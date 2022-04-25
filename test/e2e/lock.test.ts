// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

import * as PeerRunner from "test/support/peerRunner";
import * as Support from "test/support";

test("opens on LockScreen when an identity exists", async ({
  page,
}, testInfo) => {
  const stateDir = await Support.prepareStateDir(testInfo.file, testInfo.title);

  const peer = await PeerRunner.UpstreamPeer.create({
    dataPath: stateDir,
    name: "peer",
  });
  await peer.start();
  await page.goto(peer.uiUrl);

  // Expect to be on the LockScreen.
  await expect(page).toHaveURL(new RegExp("#/lock"));
});

test("shows an error notification if an incorrect passphrase is entered", async ({
  page,
}, testInfo) => {
  const stateDir = await Support.prepareStateDir(testInfo.file, testInfo.title);

  const peer = await PeerRunner.UpstreamPeer.create({
    dataPath: stateDir,
    name: "peer",
  });
  await peer.start();
  await page.goto(peer.uiUrl);

  // Click [placeholder="Enter your passphrase"]
  await page.locator('[placeholder="Enter your passphrase"]').click();
  // Fill [placeholder="Enter your passphrase"]
  await page
    .locator('[placeholder="Enter your passphrase"]')
    .fill("wrong-password");
  // Click button:has-text("Unlock")
  await page.locator('button:has-text("Unlock")').click();

  // Expect a notification showing an error message.
  await expect(page.locator("text=That’s the wrong passphrase.")).toBeVisible();
});

test("opens the ProfileScreen on successful unlock", async ({
  page,
}, testInfo) => {
  const stateDir = await Support.prepareStateDir(testInfo.file, testInfo.title);

  const peer = await PeerRunner.UpstreamPeer.create({
    dataPath: stateDir,
    name: "peer",
  });
  await peer.start();
  await page.goto(peer.uiUrl);

  // Click [placeholder="Enter your passphrase"]
  await page.locator('[placeholder="Enter your passphrase"]').click();
  // Fill [placeholder="Enter your passphrase"]
  await page.locator('[placeholder="Enter your passphrase"]').fill("asdf");
  // Press Enter
  await page.locator('[placeholder="Enter your passphrase"]').press("Enter");

  // Expect to be on the ProfileScreen.
  await expect(page).toHaveURL(new RegExp("#/profile"));
});
