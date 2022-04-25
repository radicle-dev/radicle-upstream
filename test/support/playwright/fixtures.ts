// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as path from "path";
import { test as base } from "@playwright/test";

import * as PeerRunner from "test/support/peerRunner";

export const test = base.extend<{
  forAllTests: void;
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
});

export { expect } from "@playwright/test";
