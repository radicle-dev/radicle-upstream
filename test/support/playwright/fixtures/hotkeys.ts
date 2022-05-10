// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { Page } from "@playwright/test";

export class Hotkeys {
  #page: Page;

  public constructor(page: Page) {
    this.#page = page;
  }

  public async openSearchModal(): Promise<void> {
    await this.#page.locator("body").press(`${modifierKey()}+p`);
  }

  public async closeModal(): Promise<void> {
    await this.#page.locator("body").press("Escape");
  }
}

function modifierKey(): "Meta" | "Control" {
  if (process.platform === "linux") {
    return "Control";
  } else {
    return "Meta";
  }
}
