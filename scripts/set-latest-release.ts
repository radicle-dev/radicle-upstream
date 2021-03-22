#!/usr/bin/env -S yarn run ts-node

// This script updates `releases.radicle.xyz/latest.json` with the
// current.
//
// This script runs `gsutil` so you need to be logged into an account
// that has permissions to write to the `releases.radicle.xyz` bucket.

import * as os from "os";
import * as path from "path";
import * as fs from "fs";
import * as childProcess from "child_process";

import * as semver from "semver";
import fetch from "node-fetch";

const pkg = require("../package.json");

const fileName = "latest.json";

main().catch(e => {
  console.error(e);
  process.exit(1);
});

async function main() {
  if (!semver.gte(process.version, "14.14.0")) {
    throw new Error(
      "You’re using an outdated version of Node. This script requires at least v14.14.0"
    );
  }

  await withTempDir(async tempDir => {
    const versionDash = pkg.version.replace(/\./g, "-");
    const announcementUrl = `https://radicle.community/t/radicle-upstream-v${versionDash}-is-out`;
    const response = await fetch(
      `https://radicle.community/t/radicle-upstream-v${versionDash}-is-out`
    );
    if (!response.ok) {
      throw new Error(
        `Announcement url ${announcementUrl} does not exist. Response status is ${response.status}`
      );
    }

    const latestPath = path.join(tempDir, fileName);
    await fs.promises.writeFile(
      latestPath,
      JSON.stringify(
        {
          version: pkg.version,
          announcementUrl,
        },
        null,
        2
      ),
      "utf8"
    );

    const result = childProcess.spawnSync(
      "gsutil",
      ["cp", latestPath, `gs://releases.radicle.xyz/${fileName}`],
      { stdio: "inherit" }
    );

    if (result.error) {
      throw result.error;
    }

    if (result.signal !== null) {
      throw new Error(`gsutil killed by signal ${result.signal}`);
    }

    if (result.status !== 0) {
      throw new Error(`gsutil exited with status code ${result.status}`);
    }
  });
}

async function withTempDir(
  cb: (tempDir: string) => Promise<void>
): Promise<void> {
  const tempDir = await fs.promises.mkdtemp(
    path.join(os.tmpdir(), "radicle-dev-set-latest-release-")
  );
  try {
    await cb(tempDir);
  } finally {
    await fs.promises.rm(tempDir, { recursive: true });
  }
}
