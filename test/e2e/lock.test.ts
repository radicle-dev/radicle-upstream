// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

test.beforeEach(async ({ page, peerManager }) => {
  const peer = await peerManager.startPeer({
    name: "peer",
    disableSshAgent: true,
  });
  await page.goto(peer.uiUrl);
});

test("show LockScreen when an identity exists", async ({ page }) => {
  // Expect to be on the LockScreen.
  await expect(page).toHaveURL(new RegExp("#/lock"));
});

test("show error notification if passphrase is incorrect", async ({ page }) => {
  await page.locator('[placeholder="Enter your passphrase"]').click();
  await page
    .locator('[placeholder="Enter your passphrase"]')
    .fill("wrong-password");
  await page.locator('button:has-text("Unlock")').click();

  // Expect a notification showing an error message.
  await expect(page.locator("text=That’s the wrong passphrase.")).toBeVisible();
});

test("open ProfileScreen after successful unlock", async ({ page }) => {
  await page.locator('[placeholder="Enter your passphrase"]').click();
  await page.locator('[placeholder="Enter your passphrase"]').fill("asdf");
  await page.locator('[placeholder="Enter your passphrase"]').press("Enter");

  // Expect to be on the ProfileScreen.
  await expect(page).toHaveURL(new RegExp("#/profile"));
});
