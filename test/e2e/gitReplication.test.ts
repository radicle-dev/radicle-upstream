// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";

import * as Support from "test/support";
import * as PeerRunner from "test/support/peerRunner";
import * as Helpers from "test/support/playwright/helpers";

test("contributor follows", async ({ page }, testInfo) => {
  const stateDir = await Support.prepareStateDir(testInfo.file, testInfo.title);
  const sshAuthSock = await Support.startSshAgent();

  const maintainer = await PeerRunner.UpstreamPeer.create({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock,
  });
  await maintainer.start();
  const projectUrn = await Support.createProject(maintainer, "foo");

  const contributor = await PeerRunner.UpstreamPeer.create({
    dataPath: stateDir,
    name: "contributor",
    sshAuthSock,
  });

  await contributor.start();
  await page.goto(contributor.uiUrl);

  // Press p with modifiers
  await page.locator("body").press(`${Helpers.modifierKey()}+p`);
  // Fill [placeholder="Enter a Project ID here…"]
  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill(projectUrn);
  // Press Enter
  await page.locator('[placeholder="Enter a Project ID here…"]').press("Enter");

  await expect(page.locator(".project-card h2")).toHaveText("foo");
});
