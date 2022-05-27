// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { test, expect } from "test/support/playwright/fixtures";
import * as Support from "test/support";
import { Page } from "@playwright/test";

test("show our own peer", async ({ app, page, peerManager }) => {
  const peer = await peerManager.startPeer({ name: "peer" });
  const projectName = "foo";
  await Support.createProject(peer, projectName);

  await page.goto(peer.uiUrl);
  await app.goToProjectByName(projectName);
  await app.projectScreen.goToPatchesTab();
  await expect(app.projectScreen.selectPeerButton).toContainText(
    `${peer.userHandle} delegate`
  );
});

test("add a new peer and peer input validation", async ({
  app,
  page,
  peerManager,
}) => {
  const maintainer = await peerManager.startPeer({ name: "maintainer" });
  const projectName = "foo";
  const { urn: projectUrn } = await Support.createAndPublishProject(
    maintainer,
    projectName
  );

  const contributor = await peerManager.startPeer({ name: "contributor" });

  await Support.cloneProject(contributor, projectUrn, projectName);

  await page.goto(maintainer.uiUrl);
  await app.goToProjectByName(projectName);
  await app.projectScreen.editRemotePeersButton.click();

  const editRemotesDialog = makeEditRemoteDialog(page);
  await editRemotesDialog.peerInput.fill(contributor.peerId);
  await editRemotesDialog.addPeerButton.click();

  // Add peer ID a second time
  {
    await editRemotesDialog.peerInput.fill(contributor.peerId);
    await editRemotesDialog.addPeerButton.click();
    await expect(editRemotesDialog.self).toContainText(
      "This remote is already being tracked"
    );
    await editRemotesDialog.peerInput.fill("");
    await expect(editRemotesDialog.self).not.toContainText(
      "This remote is already being tracked"
    );
  }

  // Try to add invalid peer ID
  {
    await editRemotesDialog.peerInput.fill(`${contributor.peerId}foo`);
    await editRemotesDialog.addPeerButton.click();
    await expect(editRemotesDialog.self).toContainText(
      "This is not a valid remote"
    );
  }

  // Untrack peer
  {
    const peerItem = editRemotesDialog.peerItem(contributor.peerId);
    await peerItem.locator('role=button[name="Tracking"]').click();
    await expect(peerItem).not.toBeVisible();
  }
});

function makeEditRemoteDialog(page: Page) {
  const self = page.locator('role=dialog[name="Edit remotes"]');
  return {
    self,
    peerInput: self.locator('[placeholder="Enter a Peer ID here"]'),
    addPeerButton: self.locator('role=button[name="Add"]'),
    peerItem(peerId: string) {
      return self.locator("role=listitem", {
        hasText: peerId.substring(0, 7),
      });
    },
  };
}
