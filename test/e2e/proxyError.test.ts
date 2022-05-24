// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as IpcTypes from "native/ipc-types";
import { test, expect } from "test/support/playwright/fixtures";

test("show error screen if app receives a proxy error", async ({
  page,
  app,
  peerManager,
}) => {
  const peer = await peerManager.startPeer({ name: "peer" });
  await page.goto(peer.uiUrl);

  await app.sendElectronMessage({
    kind: IpcTypes.MainMessageKind.PROXY_ERROR,
    data: {
      status: 1,
      signal: null,
      output: "OUTPUT",
    },
  });
  await expect(page.locator("body")).toContainText(
    "Hmm, looks like the app can’t be loaded right now because the backend has crashed or it isn’t starting."
  );
  await expect(page.locator("body")).toContainText("OUTPUT");
  await page.locator('role=button[name="Copy to clipboard"]').click();
  const clipboard = await page.evaluate(() => {
    return window.electronMainProcessStubs.clipboardWriteText.args[0][0];
  });
  expect(clipboard).toEqual("OUTPUT");
});
