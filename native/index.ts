// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import {
  app,
  BrowserWindow,
  ipcMain,
  dialog,
  clipboard,
  shell,
} from "electron";
import fs from "fs";
import path from "path";
import qs from "qs";
import execa from "execa";
import {
  ProxyProcessManager,
  Options as ProxyProcessOptions,
} from "./proxy-process-manager";
import {
  MainMessage,
  MainMessageKind,
  MainProcess,
  mainProcessMethods,
} from "./ipc-types";
import { parseRadicleUrl, throttled } from "./nativeCustomProtocolHandler";
import type { Config as UiConfig } from "ui/src/config";
import { config, Config } from "./config";

const isWindows = process.platform === "win32";

if (config.radHome) {
  const electronPath = path.resolve(config.radHome, "electron");
  fs.mkdirSync(electronPath, { recursive: true });
  app.setPath("userData", electronPath);
  app.setPath("appData", electronPath);
}

class WindowManager {
  public window: BrowserWindow | null;
  private messages: MainMessage[];

  public constructor() {
    this.window = null;
    this.messages = [];
  }

  // Send a message on the "message" channel to the renderer window
  public sendMessage(message: MainMessage) {
    if (this.window === null || this.window.webContents.isLoading()) {
      this.messages.push(message);
    } else {
      this.window.webContents.send("message", message);
    }
  }

  public reload() {
    if (this.window) {
      this.window.reload();
    }
  }

  public open() {
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

  public focus() {
    if (!this.window) {
      return;
    }

    if (this.window.isMinimized()) {
      this.window.restore();
    }

    this.window.focus();
  }

  public close() {
    if (this.window) {
      this.window.close();
    }
  }
}

function proxyProcessOptions(config: Config): ProxyProcessOptions {
  let proxyPath;
  let proxyArgs: string[] = [];

  if (config.environment === "development") {
    proxyPath = path.join(__dirname, "../target/debug/radicle-proxy");

    proxyArgs = [
      "--skip-remote-helper-install",
      "--unsafe-fast-keystore",
      "--dev-log",
      "--http-listen",
      config.httpAddr,
    ];
  } else {
    // Packaged app, i.e. production.
    if (isWindows) {
      proxyPath = path.join(__dirname, "../../radicle-proxy.exe");
    } else {
      proxyPath = path.join(__dirname, "../../radicle-proxy");
    }
  }

  return {
    proxyPath,
    proxyArgs,
    lineLimit: 500,
    env: {
      RAD_HOME: config.radHome,
    },
  };
}

const windowManager = new WindowManager();
const proxyProcessManager = new ProxyProcessManager(
  proxyProcessOptions(config)
);

function installMainProcessHandler(handler: MainProcess) {
  mainProcessMethods.forEach(method => {
    ipcMain.handle(method, async (_event, arg) => handler[method](arg));
  });
}

installMainProcessHandler({
  async getProxyLogs(): Promise<string> {
    return proxyProcessManager.getOutputBuffer();
  },
  async clipboardWriteText(text: string): Promise<void> {
    clipboard.writeText(text);
  },
  async getVersion(): Promise<string> {
    if (config.environment === "development") {
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      const version = require("../package.json")["version"];
      const { stdout, stderr } = await execa("git", ["rev-parse", "HEAD"]);

      if (!version || stderr) {
        return "0.0.0";
      } else {
        return `${version}-${stdout.trim()}`;
      }
    } else {
      return app.getVersion();
    }
  },
  async openPath(path: string): Promise<void> {
    shell.openPath(path);
  },
  async openUrl(url: string): Promise<void> {
    openExternalLink(url);
  },
  async getGitGlobalDefaultBranch(): Promise<string | undefined> {
    try {
      const { stdout, stderr } = await execa("git", [
        "config",
        "--global",
        "--get",
        "init.defaultBranch",
      ]);
      return stderr ? undefined : stdout.trim();
    } catch (error: unknown) {
      return undefined;
    }
  },
  async selectDirectory(): Promise<string> {
    const window = windowManager.window;
    if (window === null) {
      return "";
    }

    const result = await dialog.showOpenDialog(window, {
      properties: ["openDirectory", "showHiddenFiles", "createDirectory"],
    });

    if (result.filePaths.length === 1) {
      return result.filePaths[0];
    } else {
      return "";
    }
  },
});

function setupWatcher() {
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const chokidar = require("chokidar");
  const watcher = chokidar.watch(path.join(__dirname, "../public/**"), {
    ignoreInitial: true,
  });

  watcher.on("change", () => {
    windowManager.reload();
  });
}

const openExternalLink = (url: string): void => {
  if (
    url.toLowerCase().startsWith("http://") ||
    url.toLowerCase().startsWith("https://")
  ) {
    shell.openExternal(url);
  } else {
    console.warn(`User tried opening URL with invalid URI scheme: ${url}`);
  }
};

app.on("render-process-gone", (_event, _webContents, details) => {
  if (details.reason !== "clean-exit") {
    console.error(`Electron render process is gone. Reason: ${details.reason}`);
    shutdown();
  }
});

app.on("before-quit", event => {
  windowManager.close();
  event.preventDefault();
  shutdown();
});

// Handle custom protocol on macOS
app.on("open-url", (event, url) => {
  event.preventDefault();

  const parsedUrl = parseRadicleUrl(url);
  if (parsedUrl) {
    throttled(() => {
      windowManager.sendMessage({
        kind: MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
        data: { url: parsedUrl },
      });
    });
  }
});

if (app.requestSingleInstanceLock()) {
  // Handle custom protocol on Linux when Upstream is already running
  app.on("second-instance", (_event, argv, _workingDirectory) => {
    const parsedUrl = parseRadicleUrl(argv[1]);
    if (parsedUrl) {
      throttled(() => {
        windowManager.focus();
        windowManager.sendMessage({
          kind: MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
          data: { url: parsedUrl },
        });
      });
    }
  });

  // Handle custom protocol on Linux when Upstream is not running
  const parsedUrl = parseRadicleUrl(process.argv[1]);
  if (parsedUrl) {
    throttled(() => {
      windowManager.sendMessage({
        kind: MainMessageKind.CUSTOM_PROTOCOL_INVOCATION,
        data: { url: parsedUrl },
      });
    });
  }

  // This method will be called when Electron has finished
  // initialization and is ready to create browser windows.
  // Some APIs can only be used after this event occurs.
  app.on("ready", () => {
    process.on("SIGINT", () => {
      shutdown();
    });

    process.on("SIGTERM", () => {
      shutdown();
    });

    proxyProcessManager.run().then(({ status, signal, output }) => {
      windowManager.sendMessage({
        kind: MainMessageKind.PROXY_ERROR,
        data: {
          status,
          signal,
          output,
        },
      });
    });

    if (config.environment === "development") {
      setupWatcher();
    }

    windowManager.open();
  });
} else {
  app.quit();
}

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") {
    shutdown();
  }
});

app.on("activate", () => {
  if (app.isReady() && !windowManager.window) {
    windowManager.open();
  }
});

let isShuttingDown = false;

async function shutdown() {
  if (isShuttingDown) {
    return;
  }

  isShuttingDown = true;
  await proxyProcessManager.shutdown().catch(e => console.error(e));
  app.exit();
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
  return uiConfig;
}
