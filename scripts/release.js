#!/usr/bin/env node

const util = require("util");
const exec = util.promisify(require("child_process").exec);

const verboseExec = async cmd => {
  const result = await exec(cmd);
  console.log(` ✔ ${cmd}`);

  return result;
};

const VERSION_MATCH = /bumping version in package.json from (.*) to (.*)/;
const PULL_REQUEST_MATCH =
  "https://github.com/radicle-dev/radicle-upstream/pull/(.*)";

const SV_COMMAND = "yarn run standard-version --infile ./CHANGELOG.md";

const checkPrerequisites = async () => {
  try {
    await exec("hub --version");
  } catch (error) {
    if (error.stderr.match("command not found")) {
      printMissingDependenciesMsg();
    } else {
      throw error;
    }
  }
};

const printMissingDependenciesMsg = () =>
  console.log(`
  Please install missing dependencies:
    - https://github.com/github/hub
`);

const finalizeRelease = async () => {
  const toVersion = process.argv[3];
  const pullRequestId = process.argv[4];

  if (toVersion === undefined || pullRequestId === undefined) {
    printWrongArgsMsg();
    return;
  }

  console.log(`Finalizing release ${toVersion}:\n`);

  const mergeResult = await verboseExec(
    `hub api -XPUT ` +
      `"repos/radicle-dev/radicle-upstream/pulls/${pullRequestId}/merge" ` +
      `--raw-field 'merge_method=squash'`
  );
  const releaseCommitSHA = JSON.parse(mergeResult.stdout).sha;
  await verboseExec("git checkout master && git pull");
  await verboseExec(`git tag ${toVersion} ${releaseCommitSHA}`);
  await verboseExec(`git push --tags`);

  console.log(`\nRelease ${toVersion} successfully completed! 👏 🎉 🚀\n`);
};

const printWrongArgsMsg = () =>
  console.log(`
  This command should not be run stand-alone.
  You should run \`yarn release\` and follow the instructions.
`);

const cutRelease = async toVersion => {
  console.log(`\nCutting release ${toVersion}:\n`);

  await verboseExec("git checkout master");
  await verboseExec(
    `git branch release-${toVersion} && git checkout release-${toVersion}`
  );
  await verboseExec(SV_COMMAND);
  await verboseExec(`git push origin release-${toVersion}`);
  const prResult = await verboseExec("hub pull-request -p --no-edit");
  const prUrl = prResult.stdout.split("\n").slice(-2)[0];
  const pullRequestId = prUrl.match(PULL_REQUEST_MATCH)[1];

  printNextStepsMsg(prUrl, toVersion, pullRequestId);
};

const printNextStepsMsg = (prUrl, toVersion, pullRequestId) =>
  console.log(`
  Now fix up CHANGELOG.md if necessary and update QA.md
  to cover the latest changes in functionality.

  When everything is in shape, ask a peer to review the
  pull request, but don't merge it via the GitHub UI:

    👉 ${prUrl}

  Finally, complete the release by running:

    👉 yarn release --finalize ${toVersion} ${pullRequestId}
`);

async function main() {
  checkPrerequisites();

  const svResult = await exec(`${SV_COMMAND} --dry-run`);
  const toVersion = `v${svResult.stdout.match(VERSION_MATCH)[2]}`;

  if (process.argv[2] === "--finalize") {
    finalizeRelease();
  } else {
    cutRelease(toVersion);
  }
}

main();
