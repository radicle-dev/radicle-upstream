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
    async ({ context }, use) => {
      await context.addInitScript({
        path: path.join(__dirname, "ipcStub.js"),
      });

      await use();
      Process.killAllProcesses();
    },
    { scope: "test", auto: true },
  ],
  // eslint-disable-next-line no-empty-pattern
  peerManager: async ({}, use, testInfo) => {
    const stateDir = await Support.prepareStateDir(
      testInfo.file,
      testInfo.title
    );
    const peerManager = await createPeerManager({ dataPath: stateDir });
    await use(peerManager);
    await peerManager.teardown();
  },
  app: async ({ page }, use) => {
    await use(new App(page));
  },
});

export { expect } from "@playwright/test";
