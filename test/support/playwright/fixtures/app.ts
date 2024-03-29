// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { Page, Locator } from "@playwright/test";
import * as IpcTypes from "native/ipc-types";

import { Hotkeys } from "./hotkeys";
import { ProjectScreen } from "./projectScreen";

export class App {
  public hotkeys: Hotkeys;
  public projectScreen: ProjectScreen;
  public projectList: Locator;

  #page: Page;

  public constructor(page: Page) {
    this.#page = page;

    this.hotkeys = new Hotkeys(page);
    this.projectScreen = new ProjectScreen(page);
    this.projectList = this.#page.locator("[data-cy=project-list]");
  }

  public async goToProjectByName(name: string): Promise<void> {
    await this.#page
      .locator('[data-cy="sidebar"] >> [data-cy="profile"]')
      .click();
    await this.projectList.locator(`text=${name}`).click();
  }

  public async trackProject(projectId: string): Promise<void> {
    this.hotkeys.openSearchModal();
    await this.#page
      .locator('[placeholder="Enter a Project ID here…"]')
      .fill(projectId);
    await this.#page
      .locator('[placeholder="Enter a Project ID here…"]')
      .press("Enter");
  }

  public async goToSetting(): Promise<void> {
    await this.#page
      .locator('role=navigation[name="main"]')
      .locator('role=button[name="Settings"]')
      .click();
  }

  public async sendElectronMessage(
    message: IpcTypes.MainMessage
  ): Promise<void> {
    await this.#page.evaluate(message => {
      window.electronMainProcessStubs.sendMessage(message);
    }, message);
  }

  public async openRadicleUrl(url: string): Promise<void> {
    await this.sendElectronMessage({
      kind: IpcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
      data: {
        url,
      },
    });
  }

  public async getClipboardContents(): Promise<string> {
    return await this.#page.evaluate(() => {
      return window.electronMainProcessStubs.clipboardWriteText.args[0][0];
    });
  }
}
