// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { Page, Locator } from "@playwright/test";
import { Hotkeys } from "./hotkeys";

export class ProjectScreen {
  public actionBar: Locator;
  public commitPage: Locator;
  public patchCounter: Locator;
  public patchList: Locator;
  public patchPage: Locator;
  public editRemotePeersButton: Locator;
  public selectPeerButton: Locator;

  #page: Page;
  #hotkeys: Hotkeys;

  public constructor(page: Page) {
    this.#page = page;
    this.#hotkeys = new Hotkeys(page);

    this.actionBar = this.#page.locator("[data-cy=action-bar]");
    this.patchList = this.#page.locator("[data-cy=patch-list]");
    this.patchPage = this.#page.locator("[data-cy=patch-page]");
    this.commitPage = this.#page.locator("[data-cy=commit-page]");
    this.patchCounter = this.#page.locator(
      `[data-cy="patches-tab"] [data-cy="counter"]`
    );
    this.selectPeerButton = page.locator('role=button[name="select peer"]');
    this.editRemotePeersButton = page.locator(
      'role=button[name="edit remote peers"]'
    );
  }

  public async goToPatchesTab(): Promise<void> {
    await this.#page.locator(`[data-cy="patches-tab"]`).click();
  }

  public async goToPatchByTitle(title: string): Promise<void> {
    await this.patchList.locator(`text=${title}`).click();
  }

  public async addRemotes(peerIds: string[]): Promise<void> {
    await this.#page.locator("button[data-cy=manage-remotes]").click();
    for (const peerId of peerIds) {
      await this.#page
        .locator('[placeholder="Enter a Peer ID here"]')
        .fill(peerId);
      await this.#page
        .locator('[data-cy=remotes-modal] button:has-text("Add")')
        .click();
    }
    this.#hotkeys.closeModal();
  }
}
