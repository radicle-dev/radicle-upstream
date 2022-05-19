// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { BrowserWindow, shell } from "electron";
import path from "path";
import qs from "qs";
import { MainMessage } from "./ipc-types";
import type { Config as UiConfig } from "ui/src/config";
import { config, Config } from "./config";

export function openExternalLink(url: string): void {
  if (
    url.toLowerCase().startsWith("http://") ||
    url.toLowerCase().startsWith("https://")
  ) {
    shell.openExternal(url);
  } else {
    console.warn(`User tried opening URL with invalid URI scheme: ${url}`);
  }
}

export class WindowManager {
  public window: BrowserWindow | null;
  private messages: MainMessage[];

  public constructor() {
    this.window = null;
    this.messages = [];
  }

  // Send a message on the "message" channel to the renderer window
  public sendMessage(message: MainMessage): void {
    if (this.window === null || this.window.webContents.isLoading()) {
      this.messages.push(message);
    } else {
      this.window.webContents.send("message", message);
    }
  }

  public reload(): void {
    if (this.window) {
      this.window.reload();
    }
  }

  public open(): void {
    if (this.window) {
      return;
    }

    const window = new BrowserWindow({
      width: 1200,
      height: 680,
      icon: path.join(__dirname, "../public/icon.png"),
      show: false,
      autoHideMenuBar: true,
      webPreferences: {
        preload: path.join(__dirname, "preload.js"),
        contextIsolation: true,
      },
    });

    window.once("ready-to-show", () => {
      window.maximize();
      window.show();
    });

    window.webContents.on("will-navigate", (event, url) => {
      event.preventDefault();
      openExternalLink(url);
    });

    window.webContents.on("new-window", (event, url) => {
      event.preventDefault();
      openExternalLink(url);
    });

    window.on("closed", () => {
      this.window = null;
    });

    window.webContents.on("did-finish-load", () => {
      this.messages.forEach(message => {
        window.webContents.send("message", message);
      });
      this.messages = [];
    });

    const query = qs.stringify({
      config: JSON.stringify(buildUiConfig(config)),
    });

    const htmlPath = path.resolve(__dirname, "..", "public", "index.html");
    window.loadURL(`file://${htmlPath}?${query}`);

    this.window = window;
  }

  public focus(): void {
    if (!this.window) {
      return;
    }

    if (this.window.isMinimized()) {
      this.window.restore();
    }

    this.window.focus();
  }

  public close(): void {
    if (this.window) {
      this.window.close();
    }
  }
}

function buildUiConfig(config: Config): Partial<UiConfig> {
  const uiConfig: Partial<UiConfig> = {
    proxyAddress: config.httpAddr,
  };
  if (config.environment === "development") {
    uiConfig.isDev = true;
  }
  if (config.testWalletMnemonic) {
    uiConfig.testWalletMnemonic = config.testWalletMnemonic;
  }
  uiConfig.path = config.path;
  return uiConfig;
}
