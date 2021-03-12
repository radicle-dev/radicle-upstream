#!/usr/bin/env -S npx ts-node -P tsconfig.scripts.json

import { execSync } from "child_process";
import prompts from "prompts";
import * as semver from "semver";
import standardVersion from "standard-version";

const CURRENT_RELEASE: string = require("../package.json").version;

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
  const minHubVersion = "2.14.0";
  let result;

  try {
    result = execSync("hub --version", {
      stdio: "pipe",
    }).toString("utf-8");
  } catch {
    console.log(
      `\nCould not find the hub CLI tool >= ${minHubVersion}.\n` +
        "You can get it here:\n\n" +
        "  https://github.com/github/hub\n"
    );
    process.exit(1);
  }

  const match = result.match("hub version (.*)");

  if (match) {
    const version = semver.parse(match[1]);

    if (!version) {
      throw new Error("Couldn't parse the version number of the hub CLI tool");
    }

    if (semver.lt(version, minHubVersion)) {
      console.log(
        `\nPlease upgrade your hub CLI tool to the minimum required version of ${minHubVersion}.\n`
      );
      process.exit(1);
    }
  } else {
    throw new Error(
      "Couldn't find version number in the `hub --version` output"
    );
  }
};

const continueOnConfirm = async () => {
  const response = await prompts({
    type: "confirm",
    name: "value",
    message: "Are you sure you want to continue?",
    initial: false,
  });

  if (!response.value) {
    process.exit();
  }
};

const finalizeRelease = (version: string, pullRequestId: string) => {
  console.log(`\nFinalizing release v${version}:\n`);

  const mergeResult = verboseExec(
    `hub api -XPUT ` +
      `"repos/radicle-dev/radicle-upstream/pulls/${pullRequestId}/merge" ` +
      `--raw-field 'merge_method=squash'`
  );
  const releaseCommitSHA = JSON.parse(mergeResult).sha;
  verboseExec("git checkout master && git pull");
  verboseExec(`git tag v${version} ${releaseCommitSHA}`);
  verboseExec(`git push --tags`);

  console.log(`\nRelease v${version} successfully finalized.\n`);
};

const cutRelease = async (
  version: string,
  releaseAs: string
): Promise<void> => {
  console.log(`\nCurrent Upstream version: v${CURRENT_RELEASE}`);
  console.log(`You're about to cut a new version: v${version}\n`);

  await continueOnConfirm();

  console.log(`\nCutting release v${version}:\n`);

  verboseExec("git checkout master");
  verboseExec(
    `git branch release-v${version} && git checkout release-v${version}`
  );

  await standardVersion({
    infile: "./CHANGELOG.md",
    silent: true,
    skip: { tag: true },
    sign: true,
    releaseAs,
  });
  console.log(
    `  ✔ standard-version --infile ./CHANGELOG.md --silent --sign --release-as ${releaseAs}`
  );

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

            yarn release finalize ${version} ${pullRequestId}

  - [ ] wait for our build servers to build the macOS and Linux release
        packages
  - [ ] upload the macOS and Linux packages to https://releases.radicle.xyz

          gsutil cp gs://builds.radicle.xyz/radicle-upstream/v${version}/dist/radicle-upstream-${version}.AppImage gs://releases.radicle.xyz
          gsutil cp gs://builds.radicle.xyz/radicle-upstream/v${version}/dist/radicle-upstream-${version}.dmg gs://releases.radicle.xyz

  - [ ] create macOS and Linux QA issues in the Upstream repo

          (echo "QA: v${version} macOS\\n"; sed 's/X.X.X/${version}/g' QA.md) | hub issue create --file -
          (echo "QA: v${version} Linux\\n"; sed 's/X.X.X/${version}/g' QA.md) | hub issue create --file -

  - [ ] wait until macOS and Linux QA is performed and passes
  - [ ] open a pull request on https://github.com/radicle-dev/radicle.xyz to update the website download links
    - [ ] Update the version number and rebuild the site

       echo -n ${version} > partials/upstream-version.mustache && make

    - [ ] open the pull request
    - [ ] deploy the website by merging in the pull request
  - [ ] announce new release on https://radicle.community/c/announcements
        ${communityAnnouncementTemplate(version)}
  - [ ] announce new release on https://matrix.radicle.community/#/room/#general:radicle.community
        ${matrixAnnouncementTemplate(version)}
  - [ ] announce the new version to all Upstream users via the in-app
        notification by running this script:
          ./scripts/set-latest-release.ts
`);

const communityAnnouncementTemplate = (version: string): string => {
  const releaseDate = new Date().toISOString().substring(0, 10);
  const changelogAnchor = `${version.replace(/\./g, "")}-${releaseDate}`;

  return `
        Subject: Radicle Upstream v${version} is out! 🎉
        Message:
        =============================================================================

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

        =============================================================================
`;
};

const matrixAnnouncementTemplate = (version: string): string => {
  const communityVersion = version.replace(/\./g, "-");

  return `
        Message:
        =============================================================================

          Radicle Upstream v${version} is out! 🎉
          https://radicle.community/t/radicle-upstream-v${communityVersion}-is-out

        =============================================================================
`;
};

const printUsageAndExit = (exitCode = 0) => {
  console.log(`
  Current Upstream version: v${CURRENT_RELEASE}

  Please run this command with one of the following arguments and
  follow the provided instructions to complete the release:

    yarn release           # to release v${semver.inc(CURRENT_RELEASE, "patch")}
    yarn release minor     # to release v${semver.inc(CURRENT_RELEASE, "minor")}
    yarn release major     # to release v${semver.inc(CURRENT_RELEASE, "major")}
`);
  process.exit(exitCode);
};

const main = async () => {
  checkPrerequisites();

  const [
    releaseAs = "patch",
    finalizeVersion,
    finalizePullRequestId,
  ] = process.argv.slice(2);

  let newVersion;

  switch (releaseAs) {
    case "patch":
    case "minor":
    case "major":
      newVersion = semver.inc(CURRENT_RELEASE, releaseAs);
      if (!newVersion) {
        throw new Error("Could not increment current version");
      }

      await cutRelease(newVersion, releaseAs);
      break;

    case "finalize":
      if (
        finalizeVersion === undefined ||
        finalizePullRequestId === undefined
      ) {
        printUsageAndExit(1);
      }
      finalizeRelease(finalizeVersion, finalizePullRequestId);
      break;

    default:
      printUsageAndExit();
  }
};

main();
