// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Fs from "node:fs/promises";
import * as Path from "node:path";
import * as Stream from "node:stream";
import { test as base } from "@playwright/test";

import * as Process from "test/support/process";
import * as Support from "test/support";
import { App } from "./fixtures/app";
import { createPeerManager, PeerManager } from "../peerManager";

export const test = base.extend<{
  forAllTests: void;
  app: App;
  outputLog: Stream.Writable;
  stateDir: string;
  peerManager: PeerManager;
}>({
  forAllTests: [
    async ({ page, outputLog }, use) => {
      page.on("console", msg => {
        if (
          // Ignore messages resulting from 404 responses
          msg.text().startsWith("Failed to load resource") ||
          // Ignore errors from SSE endpoint when the page is loaded
          // or the server shuts down.
          msg.text().startsWith("Error: Received proxy peer event error")
        ) {
          return;
        }

        // The prefix is chosen so that it lines up with the prefix of
        // the stdout of the proxy processes.
        const output = prefixLines("browser      |  ", msg.text());
        outputLog.write(`${output}\n`);
        if (!process.env.CI) {
          console.log(output);
        }
      });

      page.addInitScript(() => {
        // Prevent the app from showing the update checker
        // notification.
        window.localStorage.setItem(
          "radicle.settings.updateChecker.isEnabled",
          "false"
        );
      });

      await use();
    },
    { scope: "test", auto: true },
  ],
  // eslint-disable-next-line no-empty-pattern
  stateDir: async ({}, use, testInfo) => {
    const stateDir = await Support.prepareStateDir(
      testInfo.file,
      testInfo.title
    );
    await use(stateDir);
    if (process.env.CI && testInfo.status === "passed") {
      await Fs.rm(stateDir, { recursive: true });
    }
  },
  outputLog: async ({ stateDir }, use) => {
    const logFile = await Fs.open(Path.join(stateDir, "test.log"), "a");
    await use(logFile.createWriteStream());
    await logFile.close();
  },
  peerManager: async ({ page, stateDir, outputLog }, use) => {
    const peerManager = await createPeerManager({
      dataPath: stateDir,
      outputLog: outputLog,
    });
    await use(peerManager);
    await page.close();
    await peerManager.teardown();
    Process.killAllProcesses();
  },
  app: async ({ page }, use) => {
    await use(new App(page));
  },
});

export { expect } from "@playwright/test";

function prefixLines(prefix: string, text: string) {
  return text
    .split("\n")
    .map(line => `${prefix}${line}`)
    .join("\n");
}
