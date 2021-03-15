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
import { ProxyProcessManager } from "./proxy-process-manager";
import {
  MainMessage,
  MainMessageKind,
  MainProcess,
  mainProcessMethods,
} from "./ipc-types";

const isDev = process.env.NODE_ENV === "development";

let proxyPath;
let proxyArgs: string[] = [];

if (isDev) {
  if (process.env.RADICLE_UPSTREAM_PROXY_PATH) {
    proxyPath = path.resolve(process.env.RADICLE_UPSTREAM_PROXY_PATH);
  } else {
    throw new Error(
      "RADICLE_UPSTREAM_PROXY_PATH must be set when running in dev mode!"
    );
  }

  if (process.env.RADICLE_UPSTREAM_PROXY_ARGS) {
    proxyArgs = process.env.RADICLE_UPSTREAM_PROXY_ARGS.split(/[, ]/).filter(
      Boolean
    );
  }
} else {
  // Packaged app, i.e. production.
  proxyPath = path.join(__dirname, "../../radicle-proxy");
  proxyArgs = [
    "--default-seed",
    "hynewpywqj6x4mxgj7sojhue3erucyexiyhobxx4du9w66hxhbfqbw@seedling.radicle.xyz:12345",
  ];
}

if (process.env.RAD_HOME) {
  const electronPath = path.resolve(process.env.RAD_HOME, "electron");
  fs.mkdirSync(electronPath, { recursive: true });
  app.setPath("userData", electronPath);
  app.setPath("appData", electronPath);
}

// The default value of app.allowRendererProcessReuse is deprecated, it is
// currently "false".  It will change to be "true" in Electron 9.  For more
// information please check https://github.com/electron/electron/issues/18397
app.allowRendererProcessReuse = true;

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

    window.loadURL(`file://${path.join(__dirname, "../public/index.html")}`);

    this.window = window;
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
    return app.getVersion();
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
    } catch (error) {
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
    app.quit();
  }
});

app.on("will-quit", () => {
  proxyProcessManager.kill();
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", () => {
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

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (app.isReady() && !windowManager.window) {
    windowManager.open();
  }
});

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
