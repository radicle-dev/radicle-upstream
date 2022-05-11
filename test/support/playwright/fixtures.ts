// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { test as base } from "@playwright/test";

import * as PeerRunner from "test/support/peerRunner";
import * as Support from "test/support";
import { App } from "./fixtures/app";

export const test = base.extend<{
  forAllTests: void;
  stateDir: string;
  sshAuthSock: string;
  app: App;
}>({
  forAllTests: [
    async ({ context }, use) => {
      await context.addInitScript({
        path: path.join(__dirname, "ipcStub.js"),
      });

      await use();

      PeerRunner.killAllProcesses();
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
  },
  // eslint-disable-next-line no-empty-pattern
  sshAuthSock: async ({}, use) => {
    const sshAuthSock = await Support.startSshAgent();
    await use(sshAuthSock);
  },
  app: async ({ page }, use) => {
    await use(new App(page));
  },
});

export { expect } from "@playwright/test";
