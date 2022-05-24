// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { UpstreamPeer } from "test/support/peerManager";
import { test, expect } from "test/support/playwright/fixtures";

let peer: UpstreamPeer;
test.beforeEach(async ({ peerManager, page }) => {
  peer = await peerManager.startPeer({ name: "maintainer" });
  page.addInitScript(() => {
    // By default, this is set to `false` for test.
    window.localStorage.removeItem("radicle.settings.updateChecker.isEnabled");
  });
});

test("theme switch", async ({ page, app }) => {
  await page.goto(peer.uiUrl);
  await app.goToSetting();
  await page.locator('role=button[name="Light"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");

  await page.locator('role=button[name="H4x0r"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "h4x0r");

  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "h4x0r");
});

// Current version is hardcoded to `v1.2.3` in `ipc-stub.ts`
const NEW_VERSION = "v1.2.4";

test("check app version", async ({ page }) => {
  page.route("https://releases.radicle.xyz/latest.json", route => {
    route.fulfill({
      status: 200,
      body: JSON.stringify({
        version: NEW_VERSION,
        announcementUrl: "ANNOUNCEMENT_URL",
      }),
    });
  });

  await page.goto(peer.uiUrl);

  await page.locator('role=button[name="Go to settings"]').click();
  await page.locator('role=button[name="Notify Me"]').click();
  await page
    .locator("role=status")
    .locator(`role=button[name="Check out Version ${NEW_VERSION}"]`)
    .click();

  const args = await page.evaluate(
    () => window.electronMainProcessStubs.openUrl.args
  );
  expect(args).toEqual([["ANNOUNCEMENT_URL"]]);
});
