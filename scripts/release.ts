#!/usr/bin/env -S npx ts-node -P tsconfig.scripts.json

import { execSync } from "child_process";

const VERSION_MATCH = "bumping version in package.json from (.*) to (.*)";
const PULL_REQUEST_MATCH =
  "https://github.com/radicle-dev/radicle-upstream/pull/(.*)";

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

  const match = svResult.match(VERSION_MATCH);
  let toVersion;
  if (match) {
    toVersion = `v${match[2]}`;
  } else {
    throw new Error("Could not get version");
  }

  return toVersion;
};

const finalizeRelease = () => {
  const toVersion = process.argv[3];
  const pullRequestId = process.argv[4];

  if (toVersion === undefined || pullRequestId === undefined) {
    printWrongArgsMsg();
    return;
  }

  console.log(`Finalizing release ${toVersion}:\n`);

  const mergeResult = verboseExec(
    `hub api -XPUT ` +
      `"repos/radicle-dev/radicle-upstream/pulls/${pullRequestId}/merge" ` +
      `--raw-field 'merge_method=squash'`
  );
  const releaseCommitSHA = JSON.parse(mergeResult).sha;
  verboseExec("git checkout master && git pull");
  verboseExec(`git tag ${toVersion} ${releaseCommitSHA}`);
  verboseExec(`git push --tags`);

  console.log(`\nRelease ${toVersion} successfully completed! 👏 🎉 🚀\n`);
};

const printWrongArgsMsg = () =>
  console.log(`
  This command should not be run stand-alone.
  You should run \`yarn release\` and follow the instructions.
`);

const cutRelease = (toVersion: string): void => {
  console.log(`\nCutting release ${toVersion}:\n`);

  verboseExec("git checkout master");
  verboseExec(
    `git branch release-${toVersion} && git checkout release-${toVersion}`
  );
  verboseExec(SV_COMMAND);
  verboseExec(`git push origin release-${toVersion}`);
  const prResult = verboseExec("hub pull-request -p --no-edit");
  const prUrl = prResult.split("\n").slice(-2)[0];
  const match = prUrl.match(PULL_REQUEST_MATCH);
  let pullRequestId;
  if (match) {
    pullRequestId = match[1];
  } else {
    throw new Error("Could not parse pull request ID");
  }

  printNextStepsMsg(prUrl, toVersion, pullRequestId);
};

const printNextStepsMsg = (
  prUrl: string,
  toVersion: string,
  pullRequestId: string
): void =>
  console.log(`
  Now fix up CHANGELOG.md if necessary and update QA.md
  to cover the latest changes in functionality.

  When everything is in shape, ask a peer to review the
  pull request, but don't merge it via the GitHub UI:

    👉 ${prUrl}

  Finally, complete the release by running:

    👉 yarn release --finalize ${toVersion} ${pullRequestId}
`);

const main = () => {
  checkPrerequisites();
  const toVersion = getNewVersion();

  if (process.argv[2] === "--finalize") {
    finalizeRelease();
  } else {
    cutRelease(toVersion);
  }
};

main();
