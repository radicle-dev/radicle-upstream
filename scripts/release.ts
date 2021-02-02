#!/usr/bin/env -S npx ts-node -P tsconfig.scripts.json

import { execSync } from "child_process";

const SV_COMMAND = "yarn run standard-version --infile ./CHANGELOG.md";

const verboseExec = (cmd: string) => {
  let result;

  try {
    result = execSync(cmd, { stdio: "pipe" }).toString("utf-8");
    console.log(`  ✔ ${cmd}`);
  } catch (error) {
    throw new Error(`Command \`${cmd}\` failed:\n\n${error}`);
  }

  return result;
};

const checkPrerequisites = () => {
  try {
    execSync("git --version", {
      stdio: "ignore",
    });
    execSync("hub --version", {
      stdio: "ignore",
    });
  } catch {
    throw new Error(`
  Please install missing dependencies:
    - https://git-scm.com
    - https://github.com/github/hub
`);
  }
};

const getNewVersion = (): string => {
  const svResult = execSync(`${SV_COMMAND} --dry-run`, {
    stdio: "pipe",
  }).toString("utf-8");

  let version;
  const VERSION_MATCH = "bumping version in package.json from (.*) to (.*)";
  const match = svResult.match(VERSION_MATCH);

  if (match) {
    version = match[2];
  } else {
    throw new Error("Could not get version");
  }

  return version;
};

const finalizeRelease = () => {
  const version = process.argv[3];
  const pullRequestId = process.argv[4];

  if (version === undefined || pullRequestId === undefined) {
    printWrongArgsMsg();
    return;
  }

  console.log(`Finalizing release v${version}:\n`);

  const mergeResult = verboseExec(
    `hub api -XPUT ` +
      `"repos/radicle-dev/radicle-upstream/pulls/${pullRequestId}/merge" ` +
      `--raw-field 'merge_method=squash'`
  );
  const releaseCommitSHA = JSON.parse(mergeResult).sha;
  verboseExec("git checkout master && git pull");
  verboseExec(`git tag v${version} ${releaseCommitSHA}`);
  verboseExec(`git push --tags`);

  console.log(`\nRelease v${version} successfully completed! 👏 🎉 🚀\n`);
};

const printWrongArgsMsg = () =>
  console.log(`
  This command should not be run stand-alone.
  You should run \`yarn release\` and follow the instructions.
`);

const cutRelease = (version: string): void => {
  console.log(`\nCutting release v${version}:\n`);

  verboseExec("git checkout master");
  verboseExec(
    `git branch release-v${version} && git checkout release-v${version}`
  );
  verboseExec(SV_COMMAND);
  verboseExec(`git push origin release-v${version}`);

  const prResult = verboseExec("hub pull-request -p --no-edit");
  const pullRequestUrl = prResult.split("\n").slice(-2)[0];
  const PULL_REQUEST_MATCH =
    "https://github.com/radicle-dev/radicle-upstream/pull/(.*)";
  const match = pullRequestUrl.match(PULL_REQUEST_MATCH);
  let pullRequestId;
  if (match) {
    pullRequestId = match[1];
  } else {
    throw new Error("Could not parse pull request ID");
  }

  printNextStepsMsg(pullRequestUrl, pullRequestId, version);
  printAnnouncementTemplate(version);
};

const printNextStepsMsg = (
  pullRequestUrl: string,
  pullRequestId: string,
  version: string
): void =>
  console.log(`
  To finish the release follow these steps one by one from top to bottom:

  - [x] cut the release
    - [ ] fix, commit and push any mistakes in CHANGELOG.md
    - [ ] wait for the release pull request to pass CI
    - [ ] get two approvals for the release pull request,
          but _don't_ merge it manually:

            ${pullRequestUrl}

    - [ ] finalize the release:

            yarn release --finalize v${version} ${pullRequestId}

  - [ ] build and notarize macOS package on your macOS machine:

          git checkout v${version}
          CSC_NAME="Monadic GmbH (XXXXXXXXXX)" \
          APPLE_ID="XXXXXXX@monadic.xyz" \
          APPLE_ID_PASSWORD="XXXX-XXXX-XXXX-XXXX" \
          NOTARIZE=true \
          yarn dist

  - [ ] wait for the Linux package to be built on master for the release on CI
  - [ ] upload Linux and macOS packages to https://releases.radicle.xyz

          (cd dist && curl -fLO "https://builds.radicle.xyz/radicle-upstream/v${version}/dist/radicle-upstream-${version}.AppImage")
          gsutil cp dist/radicle-upstream-${version}.AppImage gs://releases.radicle.xyz
          gsutil cp dist/radicle-upstream-${version}.dmg gs://releases.radicle.xyz

  - [ ] create macOS and Linux QA issues in the Upstream repo

          (echo "QA: v${version} macOS\n"; sed 's/X.X.X/${version}/g' QA.md) | hub issue create --file -
          (echo "QA: v${version} Linux\n"; sed 's/X.X.X/${version}/g' QA.md) | hub issue create --file -

  - [ ] wait until macOS and Linux QA is performed and passes
  - [ ] open a pull request to update the download links on our
        http://radicle.xyz website
    - [ ] deploy the updates by merging in the pull-request
  - [ ] announce new release on radicle.community (see template below 👇)
  - [ ] announce new release on the matrix #general:radicle.community channel
  - [ ] announce the new version to all Upstream users via the in-app
        notification by running this script:

          ./scripts/set-latest-release.ts
`);

const printAnnouncementTemplate = (version: string): void => {
  const releaseDate = new Date().toISOString().substring(0, 10);
  const changelogAnchor = `${version.replace(/\./, "")}-${releaseDate}`;
  const communityVersion = version.replace(/\./, "-");

  console.log(`
  ----------------------------------------------------------------------------

    URL: https://radicle.community/c/announcements


    Subject:

       Radicle Upstream v${version} is out! 🎉


    Body:

      # Radicle Upstream v${version} is out! 🎉

      You can find all the changelog for this release [here][1].

      Here are packages for all our supported platforms:

      - [macOS][2]
      - [Linux][3]

      For more information on how to use Radicle, check out our
      [documentation][4].

      For support, you can reach us in the [#support channel][5] of our Matrix
      chat or in the #help category of this forum.

      If you encounter a bug, please [open an issue][6].

      [1]: https://github.com/radicle-dev/radicle-upstream/blob/master/CHANGELOG.md#${changelogAnchor}
      [2]: https://releases.radicle.xyz/radicle-upstream-${version}.dmg
      [3]: https://releases.radicle.xyz/radicle-upstream-${version}.AppImage
      [4]: https://docs.radicle.xyz/docs/what-is-radicle.html
      [5]: https://matrix.radicle.community/#/room/#support:radicle.community
      [6]: https://github.com/radicle-dev/radicle-upstream/issues

  ----------------------------------------------------------------------------

  URL: https://matrix.radicle.community/#/room/#general:radicle.community


  Message:

    Radicle Upstream v${version} is out! 🎉
    https://radicle.community/t/radicle-upstream-${communityVersion}-is-out

  ----------------------------------------------------------------------------
`);
};

const main = () => {
  checkPrerequisites();
  const version = getNewVersion();

  if (process.argv[2] === "--finalize") {
    finalizeRelease();
  } else {
    cutRelease(version);
  }
};

main();
