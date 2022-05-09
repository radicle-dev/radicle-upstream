// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Path from "node:path";
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

  await page.locator("body").press(`${Helpers.modifierKey()}+p`);
  await page
    .locator('[placeholder="Enter a Project ID here…"]')
    .fill(projectUrn);
  await page.locator('[placeholder="Enter a Project ID here…"]').press("Enter");
  await expect(page.locator("[data-cy=project-list]")).toHaveText("foo");

  // Contributor checks out project and pushes a new commit
  {
    const contributorProjectPath = Path.join(contributor.checkoutPath, "foo");
    await contributor.spawn(
      "rad",
      ["clone", projectUrn, "--seed", "127.0.0.1:8778"],
      { cwd: contributor.checkoutPath }
    );
    // We need to push so that the default branch for the contributor
    // is present.
    // See <https://github.com/radicle-dev/radicle-upstream/issues/2795>
    await contributor.spawn("rad", ["push"], { cwd: contributorProjectPath });
    await contributor.spawn("git", ["checkout", "-b", "contributor-branch"], {
      cwd: contributorProjectPath,
    });
    await contributor.spawn(
      "git",
      ["commit", "--allow-empty", "--message", "contributor changes"],
      {
        cwd: contributorProjectPath,
      }
    );
    await contributor.spawn("rad", ["push"], {
      cwd: contributorProjectPath,
    });
  }

  // Maintainer tracks contributor and sees contributor commit
  {
    await page.goto(maintainer.uiUrl);
    await page.locator("[data-cy=project-list] >> text=foo").click();
    await page.locator("button[data-cy=manage-remotes]").click();
    await page
      .locator('[placeholder="Enter a Peer ID here"]')
      .fill(contributor.peerId);
    await page
      .locator('[data-cy=remotes-modal] button:has-text("Add")')
      .click();
    await page.locator("body").press("Escape");
    await page.locator("[data-cy=peer-selector]").click();
    await page
      .locator(
        `[data-cy="peer-dropdown-container"] >> text=${contributor.userHandle}`
      )
      .click();
    await page.locator("[data-cy=revision-selector]").click();
    await page
      .locator("[data-cy=revision-dropdown] >> text=contributor-branch")
      .click();
    await page.locator("[data-cy=tab-bar] >> text=Commits").click();
    await expect(page.locator("[data-cy=commits-page]")).toContainText(
      "contributor changes"
    );
  }
});
