// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";
import * as Support from "test/support";
import * as PeerRunner from "test/support/peerRunner";

test("show placeholder if there are no patches", async ({
  app,
  page,
  sshAuthSock,
  stateDir,
}) => {
  const peer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock: sshAuthSock,
  });
  const projectName = "foo";
  await Support.createProject(peer, projectName);

  await page.goto(peer.uiUrl);
  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();
  await expect(page.locator("[data-cy=empty-state]")).toContainText(
    "There are no patches to show at the moment."
  );
});

test("annotated patches", async ({ app, page, sshAuthSock, stateDir }) => {
  const peer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock: sshAuthSock,
  });
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

test("patch list reactivity", async ({ app, page, sshAuthSock, stateDir }) => {
  const peer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock: sshAuthSock,
  });

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

    await page.locator('button:has-text("Closed")').click();
    await expect(app.projectScreen.patchList).toContainText(newPatchTitle);
  }
});

test("patch reactivity", async ({ app, page, sshAuthSock, stateDir }) => {
  const peer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock: sshAuthSock,
  });

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

test("patch replication", async ({ app, page, sshAuthSock, stateDir }) => {
  const maintainer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "maintainer",
    sshAuthSock: sshAuthSock,
  });

  const projectName = "foo";
  const { urn: projectUrn } = await Support.createAndPublishProject(
    maintainer,
    projectName
  );

  const contributor = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "contributor",
    sshAuthSock: sshAuthSock,
  });

  const observer = await PeerRunner.UpstreamPeer.createAndStart({
    dataPath: stateDir,
    name: "observer",
    sshAuthSock: sshAuthSock,
  });

  const patchTitle = "Patch title";

  // Contributor tracks and forks the project and creates a patch.
  {
    await page.goto(contributor.uiUrl);
    await app.trackProject(projectUrn);
    await expect(app.projectList).toContainText(projectName);

    const projectWorkingCopyPath = await Support.forkProject(
      projectUrn,
      projectName,
      contributor
    );
    await Support.createOrUpdatePatch(
      patchTitle,
      "Patch description",
      contributor,
      projectWorkingCopyPath
    );

    await app.goToProjectByName(projectName);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Checkout patch")')
    ).toBeVisible();
  }

  // Observer tracks the project and contributor.
  {
    await page.goto(observer.uiUrl);
    await app.trackProject(projectUrn);
    await app.goToProjectByName(projectName);
    await app.projectScreen.addRemotes([contributor.peerId]);
    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Checkout patch")')
    ).toBeVisible();
  }

  // Maintainer tracks contributor and sees the patch.
  {
    await page.goto(maintainer.uiUrl);
    await app.goToProjectByName(projectName);
    await app.projectScreen.addRemotes([contributor.peerId]);

    await app.projectScreen.goToPatchesTab();
    await app.projectScreen.goToPatchByTitle(patchTitle);

    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Checkout patch")')
    ).toBeVisible();
    await expect(
      app.projectScreen.actionBar.locator('button:has-text("Merge patch")')
    ).toBeVisible();
  }
});
