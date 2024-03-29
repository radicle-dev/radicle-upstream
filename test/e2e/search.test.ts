// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

import * as Support from "test/support";
import { UpstreamPeer } from "test/support/peerManager";

let peer: UpstreamPeer;

test.beforeEach(async ({ page, peerManager }) => {
  peer = await peerManager.startPeer({ name: "peer" });
  await page.goto(peer.uiUrl());
});

test("search modal input validation", async ({ page, app }) => {
  await app.hotkeys.openSearchModal();

  // Paste a Peer ID instead of Project ID
  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill("hyb3j5y7paxfu5t1n9x1hpk863nyfm8apcqfoutfikyqan8jxjt35c");
  await expect(page.locator('[data-cy="search-modal"]')).toContainText(
    "You’ve entered a Peer ID instead of a Project ID."
  );

  // Paste an invalid URN
  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill("rad:git:123");
  await expect(page.locator('[data-cy="search-modal"]')).toContainText(
    "That’s not a valid Project ID."
  );

  // Pressing Enter when an invalid Project ID is entered doesn't do anything
  await page.locator('[placeholder="Enter a Project ID here…"]').press("Enter");
  await expect(page.locator('[data-cy="search-modal"]')).toBeVisible();

  // Surrounding whitespace is trimmed.
  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill("  rad:git:hnrk81fjxemf69dqc5knbmtc6frdi1n3rbh9o  ");
  await expect(page.locator('[data-cy="search-modal"]')).toContainText(
    "You’re not tracking this project yet"
  );
});

test("search for a project that is not yet tracked", async ({ app, page }) => {
  await app.hotkeys.openSearchModal();

  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill("rad:git:hnrk81fjxemf69dqc5knbmtc6frdi1n3rbh9o");
  await expect(page.locator('[data-cy="search-modal"]')).toContainText(
    "You’re not tracking this project yet"
  );

  await page.locator('[placeholder="Enter a Project ID here…"]').press("Enter");
  await expect(page.locator('[data-cy="search-modal"]')).not.toBeVisible();
  await expect(page.locator('[data-cy="notification"] >> nth=0')).toContainText(
    "You’ll be notified when this project has been found."
  );
  await expect(page.locator('[data-cy="project-list"]')).toContainText(
    "1 project you’re tracking hasn't been found yet."
  );

  // Cancel a tracking request.
  await page.locator('button:has-text("Show 1 project")').click();
  await page
    .locator('[data-cy=undiscovered-project] button:has-text("Tracking")')
    .click();
  await expect(
    page.locator('[data-cy=undiscovered-project] button:has-text("Tracking")')
  ).not.toBeVisible();

  // Test that the search input is cleared after each search.
  await app.hotkeys.openSearchModal();
  await expect(page.locator('[data-cy="search-modal"]')).toContainText("");
});

test("search for an already tracked project", async ({ app, page }) => {
  const { urn } = await Support.createProject(peer, "foo");
  await expect(
    page.locator("[data-cy=project-list] >> text=foo")
  ).toBeVisible();

  await app.hotkeys.openSearchModal();
  await page.locator('[placeholder="Enter a Project ID here…"]').fill(urn);

  // FIXME: If the Enter key is pressed before the track button is shown,
  // nothing happens. This is a bug in the search modal implementation.
  await expect(page.locator('button:has-text("Track")')).toBeVisible();

  await page.locator('[placeholder="Enter a Project ID here…"]').press("Enter");

  // Opens the project.
  await expect(
    page.locator('[data-cy="project-screen"] >> [data-cy="header"]')
  ).toContainText("foo");
});

test("search for a project using the mouse", async ({ page }) => {
  await page.locator('[data-cy="sidebar"] >> [data-cy="search"]').click();

  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill("rad:git:hnrk81fjxemf69dqc5knbmtc6frdi1n3rbh9o");

  await page.locator('button:has-text("Track")').click();
  await expect(page.locator('[data-cy="project-list"]')).toContainText(
    "1 project you’re tracking hasn't been found yet."
  );
});
