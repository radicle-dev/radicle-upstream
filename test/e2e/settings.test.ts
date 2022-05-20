// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

test.beforeEach(async ({ page, peerManager }) => {
  const peer = await peerManager.startPeer({ name: "maintainer" });
  await page.goto(peer.uiUrl);
});

test("theme switch", async ({ page, app }) => {
  await app.goToSetting();
  await page.locator('role=button[name="Light"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");

  await page.locator('role=button[name="H4x0r"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "h4x0r");

  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "h4x0r");
});
