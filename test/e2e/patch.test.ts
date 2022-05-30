// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";
import * as Support from "test/support";
import { App } from "test/support/playwright/fixtures/app";
import { Locator } from "@playwright/test";

test("show placeholder if there are no patches", async ({
  app,
  page,
  peerManager,
}) => {
  const peer = await peerManager.startPeer({ name: "maintainer" });
  const projectName = "foo";
  await Support.createProject(peer, projectName);

  await page.goto(peer.uiUrl);
  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();
  await expect(page.locator("[data-cy=empty-state]")).toContainText(
    "There are no patches to show at the moment."
  );
});

test("annotated patches", async ({ app, page, peerManager }) => {
  const peer = await peerManager.startPeer({ name: "maintainer" });
  const projectName = "foo";
  const { checkoutPath } = await Support.createProject(peer, projectName);

  await page.goto(peer.uiUrl);
  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();

  const patchTitle = "Patch title";
  const patchDescription = "Patch description";
  const commitMessage = "Commit message";
  await Support.createOrUpdatePatch(
    patchTitle,
    patchDescription,
    peer,
    checkoutPath,
    commitMessage
  );

  // Check patch list contents.
  {
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
    await expect(app.projectScreen.patchCounter).toContainText("1");
    await expect(app.projectScreen.patchList).toContainText(
      `Opened by ${peer.userHandle}`
    );
  }

  // Check patch contents.
  {
    await app.projectScreen.goToPatchByTitle(patchTitle);
    await expect(app.projectScreen.patchPage).toContainText(patchTitle);
    await expect(app.projectScreen.patchPage).toContainText(patchDescription);
    await expect(app.projectScreen.patchPage).toContainText(
      `Opened by ${peer.userHandle}`
    );
    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Checkout patch")')
    ).toBeVisible();
    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Merge patch")')
    ).toBeVisible();
    // There's only one commit.
    await expect(
      app.projectScreen.patchPage.locator(`[data-cy="commit"]`)
    ).toHaveCount(1);
    await expect(
      app.projectScreen.patchPage.locator(`[data-cy="commit"]`)
    ).toContainText(commitMessage);
  }

  // Check patch commit contents.
  {
    await app.projectScreen.patchPage.locator(`[data-cy="commit"]`).click();
    await expect(app.projectScreen.commitPage).toContainText(commitMessage);
    await app.projectScreen.commitPage
      .locator('[data-cy="back-button"]')
      .click();
    await expect(app.projectScreen.patchPage).toBeVisible();
  }

  // Patch without a description.
  {
    const patchTitle = "Patch without description";
    const patchDescription = "";
    const commitMessage = "Commit message";
    await Support.createOrUpdatePatch(
      patchTitle,
      patchDescription,
      peer,
      checkoutPath,
      commitMessage
    );
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(app.projectScreen.patchPage).toContainText(patchTitle);
    await expect(app.projectScreen.patchPage).toContainText(
      `Opened by ${peer.userHandle}`
    );
  }
});

test("patch list reactivity", async ({ app, page, peerManager }) => {
  test.setTimeout(60_000);

  const peer = await peerManager.startPeer({ name: "maintainer" });

  await page.goto(peer.uiUrl);

  const projectName = "foo";
  const { checkoutPath } = await Support.createProject(peer, projectName);

  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();

  // Create first patch.
  {
    const patchTitle = "Patch title";
    const patchDescription = "Patch description";
    const commitMessage = "Commit message";
    await Support.createOrUpdatePatch(
      patchTitle,
      patchDescription,
      peer,
      checkoutPath,
      commitMessage
    );

    await expect(app.projectScreen.patchList).toContainText(patchTitle);
    await expect(app.projectScreen.patchCounter).toContainText("1");
  }

  // Create second patch.
  {
    const patchTitle = "Another patch title";
    const patchDescription = "Second patch description";
    const commitMessage = "Commit message";
    const branchName = await Support.createOrUpdatePatch(
      patchTitle,
      patchDescription,
      peer,
      checkoutPath,
      commitMessage
    );

    await expect(app.projectScreen.patchList).toContainText(patchTitle);
    await expect(app.projectScreen.patchCounter).toContainText("2");

    const newPatchTitle = "This is a new patch title";
    await Support.createOrUpdatePatch(
      newPatchTitle,
      patchDescription,
      peer,
      checkoutPath,
      commitMessage,
      branchName
    );
    await expect(app.projectScreen.patchList).toContainText(newPatchTitle);

    // Merge second patch.
    await Support.mergeOwnPatch(peer, checkoutPath, branchName);
    await expect(app.projectScreen.patchCounter).toContainText("1");
    await expect(app.projectScreen.patchList).not.toContainText(newPatchTitle);

    await page.locator('button:has-text("Merged")').click();
    await expect(app.projectScreen.patchList).toContainText(newPatchTitle);
  }
});

test("patch reactivity", async ({ app, page, peerManager }) => {
  const peer = await peerManager.startPeer({ name: "maintainer" });

  await page.goto(peer.uiUrl);

  const projectName = "foo";
  const { checkoutPath } = await Support.createProject(peer, projectName);

  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();

  // Patch updates originating locally.
  {
    const patchTitle = "Patch title";
    const patchDescription = "Patch description";
    const commitMessage = "Commit message";
    const branchName = await Support.createOrUpdatePatch(
      patchTitle,
      patchDescription,
      peer,
      checkoutPath,
      commitMessage
    );

    await app.projectScreen.goToPatchByTitle(patchTitle);
    await expect(app.projectScreen.patchPage).toContainText(patchTitle);
    await expect(app.projectScreen.patchPage).toContainText(patchDescription);
    await expect(
      app.projectScreen.patchPage.locator(`[data-cy="commit"]`)
    ).toHaveCount(1);

    const newPatchTitle = "This is a new patch title";
    const newPatchDescription = "This is a new description";
    await Support.createOrUpdatePatch(
      newPatchTitle,
      newPatchDescription,
      peer,
      checkoutPath,
      commitMessage,
      branchName
    );
    await expect(app.projectScreen.patchPage).toContainText(newPatchTitle);
    await expect(app.projectScreen.patchPage).toContainText(
      newPatchDescription
    );
    await expect(
      app.projectScreen.patchPage.locator(`[data-cy="commit"]`)
    ).toHaveCount(2);

    // Merge patch.
    await Support.mergeOwnPatch(peer, checkoutPath, branchName);
    await expect(app.projectScreen.patchCounter).toBeHidden();
  }
});

test("patch statuses", async ({ app, page, peerManager }) => {
  const patchActions = makePatchAction(app);
  test.setTimeout(120_000);

  const maintainer = await peerManager.startPeer({ name: "maintainer" });

  const projectName = "foo";
  const { urn: projectUrn, checkoutPath } =
    await Support.createAndPublishProject(maintainer, projectName);

  const contributor = await peerManager.startPeer({ name: "contributor" });
  const observer = await peerManager.startPeer({ name: "observer" });

  const patchTitle = "Patch title";

  let branchName: string;
  let patchUrl: string;
  // Contributor tracks and forks the project, then creates a patch.
  {
    await page.goto(contributor.uiUrl);
    await app.trackProject(projectUrn);
    await expect(app.projectList).toContainText(projectName);

    const projectWorkingCopyPath = await Support.forkProject(
      projectUrn,
      projectName,
      contributor
    );
    branchName = await Support.createOrUpdatePatch(
      patchTitle,
      "Patch description",
      contributor,
      projectWorkingCopyPath
    );

    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await page
      .locator('role=button[name="Copy patch URL to clipboard"]')
      .click();
    patchUrl = await page.evaluate(() => {
      return window.electronMainProcessStubs.clipboardWriteText.args[0][0];
    });

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeVisible();
  }

  // Observer tracks the project and contributor and sees the patch.
  {
    await page.goto(observer.uiUrl);
    await app.trackProject(projectUrn);
    await app.goToProjectByName(projectName);
    await app.projectScreen.addRemotes([contributor.peerId]);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }

  // Maintainer tracks contributor and sees the patch.
  {
    await page.goto(maintainer.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.addRemotes([contributor.peerId]);

    await app.openRadicleUrl(patchUrl);

    await expect(patchActions.mergeButton).toBeVisible();
    await expect(patchActions.closeButton).toBeVisible();
  }

  // Maintainer closes the patch.
  {
    await patchActions.closeButton.click();

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );
    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);

    // The maintainer sees a Reopen button.
    await app.projectScreen.goToPatchByTitle(patchTitle);
    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.reopenButton).toBeVisible();
  }

  // Observer sees that the patch is closed.
  {
    await page.goto(observer.uiUrl);
    await app.goToProjectByName(projectName);

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );
    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);

    await app.projectScreen.goToPatchByTitle(patchTitle);
    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }

  // Contributor sees that the patch is closed.
  {
    await page.goto(contributor.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );

    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
  }

  // Contributor reopens the patch.
  {
    await app.projectScreen.goToPatchByTitle(patchTitle);
    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.reopenButton).toBeVisible();

    await patchActions.reopenButton.click();

    // Patch only shows up in the Open patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
  }

  // Maintainer sees the patch open again.
  {
    await page.goto(maintainer.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeVisible();
    await expect(patchActions.closeButton).toBeVisible();
  }

  // Observer sees the patch open again.
  {
    await page.goto(observer.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }

  // Contributor closes the patch.
  {
    await page.goto(contributor.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await patchActions.closeButton.click();

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );

    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
  }

  // Observer sees the patch closed again.
  {
    await page.goto(observer.uiUrl);
    await app.goToProjectByName(projectName);

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );
    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
  }

  // Maintainer reopens and merges the patch.
  {
    await page.goto(maintainer.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await page.locator('button:has-text("Closed")').click();

    await app.projectScreen.goToPatchByTitle(patchTitle);
    await patchActions.reopenButton.click();

    // Patch only shows up in the Open patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await Support.mergePatch(
      maintainer,
      checkoutPath,
      // TODO: get the Patch ID by copying it to the clipboard, requires IPC mocks.
      `${contributor.peerId}/${branchName}`
    );

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );

    await page.locator('button:has-text("Merged")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);

    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }

  // Observer sees the patch as merged.
  {
    await page.goto(observer.uiUrl);
    await app.goToProjectByName(projectName);

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );
    await page.locator('button:has-text("Merged")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);

    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }

  // Contributor sees that the patch is merged.
  {
    await page.goto(contributor.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();

    // Patch only shows up in the Closed patches list.
    await app.projectScreen.goToPatchesTab();
    await expect(page.locator("[data-cy=empty-state]")).toContainText(
      "There are no patches to show at the moment."
    );

    await page.locator('button:has-text("Merged")').click();
    await expect(app.projectScreen.patchList).toContainText(patchTitle);

    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(patchActions.mergeButton).toBeHidden();
    await expect(patchActions.closeButton).toBeHidden();
  }
});

interface PatchActions {
  mergeButton: Locator;
  checkoutButton: Locator;
  closeButton: Locator;
  reopenButton: Locator;
}

function makePatchAction(app: App): PatchActions {
  return {
    mergeButton: app.projectScreen.actionBar.locator('button:text("Merge")'),
    checkoutButton: app.projectScreen.actionBar.locator(
      'button:text("Checkout patch")'
    ),
    closeButton: app.projectScreen.actionBar.locator(
      'button:text("Close patch")'
    ),
    reopenButton: app.projectScreen.actionBar.locator(
      'button:text("Reopen patch")'
    ),
  };
}
