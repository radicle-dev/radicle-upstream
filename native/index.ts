// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import childProcess from "child_process";
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
import { ProxyProcessManager } from "./proxy-process-manager";
import {
  MainMessage,
  MainMessageKind,
  MainProcess,
  mainProcessMethods,
} from "./ipc-types";
import { parseRadicleUrl, throttled } from "./nativeCustomProtocolHandler";
import type { Config } from "ui/src/config";

const isDev = process.env.NODE_ENV === "development";
const isWindows = process.platform === "win32";

let proxyPath;
let proxyArgs: string[] = [];

if (isDev) {
  proxyPath = path.join(__dirname, "../target/debug/radicle-proxy");

  if (process.env.RADICLE_UPSTREAM_PROXY_ARGS) {
    proxyArgs =
      process.env.RADICLE_UPSTREAM_PROXY_ARGS.split(/[, ]/).filter(Boolean);
  }
  proxyArgs.push(
    "--default-seed",
    "hybz9gfgtd9d4pd14a6r66j5hz6f77fed4jdu7pana4fxaxbt369kg@setzling.radicle.xyz:12345",
    "--skip-remote-helper-install",
    "--unsafe-fast-keystore",
    "--dev-log"
  );
} else {
  // Packaged app, i.e. production.
  if (isWindows) {
    proxyPath = path.join(__dirname, "../../radicle-proxy.exe");
  } else {
    proxyPath = path.join(__dirname, "../../radicle-proxy");
  }
  proxyArgs = [
    "--default-seed",
    "hynkyndc6w3p8urucakobzna7sxwgcqny7xxtw88dtx3pkf7m3nrzc@sprout.radicle.xyz:12345",
  ];
}

if (isDev && !process.env.RAD_HOME) {
  process.env.RAD_HOME = path.resolve(__dirname, "..", "sandbox", "rad_home");
}

if (process.env.RAD_HOME) {
  const electronPath = path.resolve(process.env.RAD_HOME, "electron");
  fs.mkdirSync(electronPath, { recursive: true });
  app.setPath("userData", electronPath);
  app.setPath("appData", electronPath);
}

class WindowManager {
  public window: BrowserWindow | null;
  private messages: MainMessage[];

  constructor() {
    this.window = null;
    this.messages = [];
  }

  // Send a message on the "message" channel to the renderer window
  sendMessage(message: MainMessage) {
    if (this.window === null || this.window.webContents.isLoading()) {
      this.messages.push(message);
    } else {
      this.window.webContents.send("message", message);
    }
  }

  reload() {
    if (this.window) {
      this.window.reload();
    }
  }

  open() {
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
      config: JSON.stringify(buildConfig()),
    });

    const htmlPath = path.resolve(__dirname, "..", "public", "index.html");
    window.loadURL(`file://${htmlPath}?${query}`);

    this.window = window;
  }

  focus() {
    if (!this.window) {
      return;
    }

    if (this.window.isMinimized()) {
      this.window.restore();
    }

    this.window.focus();
  }

  close() {
    if (this.window) {
      this.window.close();
    }
  }
}

const windowManager = new WindowManager();
const proxyProcessManager = new ProxyProcessManager({
  proxyPath,
  proxyArgs,
  lineLimit: 500,
});

function installMainProcessHandler(handler: MainProcess) {
  mainProcessMethods.forEach(method => {
    ipcMain.handle(method, async (_event, arg) => handler[method](arg));
  });
}

installMainProcessHandler({
  async clipboardWriteText(text: string): Promise<void> {
    clipboard.writeText(text);
  },
  async getVersion(): Promise<string> {
    if (isDev) {
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      const version = require("../package.json")["version"];
      const { stdout, stderr } = await execAsync("git rev-parse HEAD");

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
      const { stdout, stderr } = await execAsync(
        "git config --global --get init.defaultBranch"
      );
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

    if (isDev) {
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

function execAsync(cmd: string): Promise<{ stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    childProcess.exec(cmd, (error, stdout, stderr) => {
      if (error) {
        reject(error);
      } else {
        resolve({ stdout, stderr });
      }
    });
  });
}

function buildConfig(): Partial<Config> {
  const config: Partial<Config> = {};
  if (process.env.RADICLE_UPSTREAM_UI_PROXY_ADDRESS) {
    config.proxyAddress = process.env.RADICLE_UPSTREAM_UI_PROXY_ADDRESS;
  }
  if (process.env.RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC) {
    config.testWalletMnemonic =
      process.env.RADICLE_UPSTREAM_TEST_WALLET_MNEMONIC;
  }
  if (isDev) {
    config.isDev = true;
  }
  return config;
}
