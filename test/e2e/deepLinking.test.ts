// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

test.beforeEach(async ({ page, peerManager }) => {
  const peer = await peerManager.startPeer({ name: "peer" });
  await page.goto(peer.uiUrl());
});

test("open search dialog for unknown project URI", async ({ page, app }) => {
  await app.openRadicleUrl(
    "radicle://link/v0/rad:git:hnrkjm5z3rwae9g3n6jhyo6kzh9eup5ku5odo"
  );
  await expect(
    page
      .locator('role=dialog[name="Search for project"]')
      .locator("role=textbox")
  ).toHaveValue("rad:git:hnrkjm5z3rwae9g3n6jhyo6kzh9eup5ku5odo");
});

test("show notification for invalid URI", async ({ page, app }) => {
  await app.openRadicleUrl("radicle://INVALID");
  await expect(page.locator("role=alert")).toContainText(
    "Could not parse the provided URL"
  );
});
