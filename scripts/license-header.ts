#!/usr/bin/env -S node -r ts-node/register/transpile-only

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import yargs from "yargs";
import * as fs from "fs/promises";
import * as Path from "path";
import execa from "execa";

// Error that is shown without a stacktrace to the user
class UserError extends Error {
  public constructor(public message: string) {
    super(message);
  }
}

async function main() {
  yargs
    .command<{ files: string[] | undefined }>({
      command: "check [files...]",
      describe: "Check presence of license headers in files",
      builder: yargs => {
        return yargs.positional("files", {
          describe:
            "Files to check. If not provided, all files are checked for a license header",
          array: true,
          type: "string",
        });
      },
      handler: async ({ files }) => {
        if (!files || files.length === 0) {
          files = await getPaths();
        }

        let failure = false;
        for (const file of files) {
          if (!requireLicenseHeader(file)) {
            continue;
          }

          const content = await fs.readFile(file, "utf8");
          if (!hasLicenseHeader(content)) {
            failure = true;
            console.error(`License missing from ${file}`);
          }
        }

        if (failure) {
          throw new UserError(
            "License headers missing. Run `./scripts/license-header.ts add` to fix this."
          );
        }
      },
    })
    .command({
      command: "add",
      describe: "Add missing license headers to files",
      handler: async () => {
        for (const path of await getPaths()) {
          const content = await fs.readFile(path, "utf8");
          if (!hasLicenseHeader(content)) {
            console.log(`Writing license to ${path}`);
            const licenseComment = makeLicenseComment(Path.extname(path));
            const fixedContent = `${licenseComment}${content}`;
            await fs.writeFile(path, fixedContent, "utf8");
          }
        }
      },
    })
    .version(false)
    .strict()
    .wrap(Math.min(100, yargs.terminalWidth()))
    // For `UserError` we don’t show the stack trace. We also don’t show the help
    // when an error is thrown.
    .fail((msg, err, yargs) => {
      if (err === undefined) {
        yargs.showHelp("error");
        console.error("");
        console.error(msg);
      } else if (err instanceof UserError) {
        console.error(err.message);
      } else {
        console.error(err);
      }
      process.exit(1);
    })
    .demandCommand().argv;
}

const licenseHeaderContent = [
  ` Copyright © ${new Date().getFullYear()} The Radicle Upstream Contributors`,
  "",
  " This file is part of radicle-upstream, distributed under the GPLv3",
  " with Radicle Linking Exception. For full terms see the included",
  " LICENSE file.",
];

function makeLicenseComment(extName: string): string {
  if (extName === ".js" || extName === ".ts" || extName === ".rs") {
    const commentLines = licenseHeaderContent.map(x => `//${x}`);
    return `${commentLines.join("\n")}\n\n`;
  } else if (extName === ".sh") {
    const commentLines = licenseHeaderContent.map(x => `#${x}`);
    return `${commentLines.join("\n")}\n\n`;
  } else if (extName === ".svelte") {
    return `<!--\n${licenseHeaderContent.join("\n")}\n-->\n`;
  } else if (extName === ".css") {
    const commentLines = licenseHeaderContent.map(x => ` *${x}`);
    return `/**\n${commentLines.join("\n")}\n */\n`;
  } else {
    throw new Error(`Unknown file extension ${extName}`);
  }
}

const EXTENSIONS = [".js", ".rs", ".sh", ".ts", ".svelte"];

// Returns true if the file at path requires a license header. This is
// `true` if the path has one of `EXTENSIONS`.
function requireLicenseHeader(path: string): boolean {
  if (path.endsWith("typings/node-fetch.d.ts")) {
    return false;
  } else {
    return EXTENSIONS.includes(Path.extname(path));
  }
}

// Returns the list of file paths that should include license headers.
//
// The list consists of all files checked into version control that
// have one of `EXTENSIONS`.
async function getPaths(): Promise<string[]> {
  const result = await execa("git", ["ls-files"]);
  const gitPaths = result.stdout.split("\n");
  return gitPaths.filter(path => {
    return EXTENSIONS.includes(Path.extname(path));
  });
}

// Pattern we use to check for the presence for the license headers in
// a given line.
const licenseHeaderPattern =
  /Copyright © \d{4} The Radicle Upstream Contributors/;

function hasLicenseHeader(fileContent: string): boolean {
  // We check the first three lines to account for shebangs and comment
  // starts.
  const head = fileContent.split("\n").slice(0, 3);
  return head.some(line => licenseHeaderPattern.test(line));
}

main();
