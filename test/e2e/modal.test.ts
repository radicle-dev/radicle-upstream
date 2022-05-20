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

test("only one modal can be opened at a time", async ({ page, app }) => {
  await app.hotkeys.openHotkeysModal();
  await expect(page.locator('[data-cy="hotkey-modal"]')).toBeVisible();

  await app.hotkeys.openSearchModal();
  await expect(page.locator('[data-cy="hotkey-modal"]')).not.toBeVisible();
  await expect(page.locator('[data-cy="search-modal"]')).toBeVisible();
});

test("navigating to a screen closes any open modals", async ({ page, app }) => {
  await app.hotkeys.openHotkeysModal();
  await expect(page.locator('[data-cy="hotkey-modal"]')).toBeVisible();

  await app.hotkeys.goToSettingsScreen();
  await expect(page.locator('[data-cy="hotkey-modal"]')).not.toBeVisible();
  await expect(page.locator("h1")).toContainText("Settings");
});
