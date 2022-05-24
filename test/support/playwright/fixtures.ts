// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { test as base } from "@playwright/test";

import * as Process from "test/support/process";
import * as Support from "test/support";
import { App } from "./fixtures/app";
import { createPeerManager, PeerManager } from "../peerManager";

export const test = base.extend<{
  forAllTests: void;
  app: App;
  peerManager: PeerManager;
}>({
  forAllTests: [
    async ({ context, page }, use) => {
      await context.addInitScript({
        path: path.join(__dirname, "ipcStub.js"),
      });
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
        console.log(prefixLines("browser      |  ", msg.text()));
      });

      await use();
      Process.killAllProcesses();
    },
    { scope: "test", auto: true },
  ],
  peerManager: async ({ page }, use, testInfo) => {
    const stateDir = await Support.prepareStateDir(
      testInfo.file,
      testInfo.title
    );
    const peerManager = await createPeerManager({ dataPath: stateDir });
    await use(peerManager);
    await page.close();
    await peerManager.teardown();
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
