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

async function main() {
  console.log();

  const svResult = await exec(`${SV_COMMAND} --dry-run`);
  const toVersion = `v${svResult.stdout.match(VERSION_MATCH)[2]}`;

  try {
    await exec("hub --version");
  } catch (error) {
    if (error.stderr.match("command not found")) {
      console.log("Please install missing dependencies:");
      console.log("  - https://github.com/github/hub");
    } else {
      throw error;
    }
  }

  if (process.argv[2] === "--finalize") {
    const toVersion = process.argv[3];
    const pullRequestId = process.argv[4];

    if (toVersion === undefined || pullRequestId === undefined) {
      console.log("This command should not be run stand-alone.");
      console.log("You should run `yarn release` and follow the instructions.");
      console.log();
      return;
    }

    console.log(`Finalizing release ${toVersion}:`);
    console.log();

    const mergeResult = await verboseExec(
      `hub api -XPUT "repos/radicle-dev/radicle-upstream/pulls/${pullRequestId}/merge" --raw-field 'merge_method=squash'`
    );
    const releaseCommitSHA = JSON.parse(mergeResult.stdout).sha;

    await verboseExec("git checkout master && git pull");

    await verboseExec(`git tag ${toVersion} ${releaseCommitSHA}`);

    await verboseExec(`git push --tags`);
    console.log();
    console.log(`Release ${toVersion} successfully completed! 👏 🎉 🚀`);
    console.log();
  } else {
    console.log(`Cutting release ${toVersion}:\n`);

    await verboseExec("git checkout master");

    await verboseExec(
      `git branch release-${toVersion} && git checkout release-${toVersion}`
    );

    await verboseExec(SV_COMMAND);
    await verboseExec(`git push origin release-${toVersion}`);

    const prResult = await verboseExec("hub pull-request -p --no-edit");

    const prUrl = prResult.stdout.split("\n").slice(-2)[0];
    const pullRequestId = prUrl.match(PULL_REQUEST_MATCH)[1];

    console.log();
    console.log("Now fix up CHANGELOG.md if necessary and update QA.md");
    console.log("to cover the latest changes in functionality.");
    console.log();
    console.log("When everything is in shape, ask a peer to review the");
    console.log("pull request, but don't merge it via the GitHub UI:");
    console.log();
    console.log(`  👉 ${prUrl}`);
    console.log();
    console.log("Finally, complete the release by running:");
    console.log();
    console.log(`  👉 yarn release --finalize ${toVersion} ${pullRequestId}`);
  }

  console.log();
}

main();
