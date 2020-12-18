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
import uuid from "uuid";
import { ProxyProcessManager } from "./proxy-process-manager";
import { RendererMessage, MainMessage, MainMessageKind } from "./ipc-types";

const isDev = process.env.NODE_ENV === "development";
let proxyPath;
if (isDev) {
  if (process.env.RADICLE_UPSTREAM_PROXY_PATH) {
    proxyPath = path.join(__dirname, process.env.RADICLE_UPSTREAM_PROXY_PATH);
  } else {
    throw new Error(
      "RADICLE_UPSTREAM_PROXY_PATH must be set when running in dev mode!"
    );
  }
} else {
  proxyPath = path.join(__dirname, "../../radicle-proxy");
}

// The default value of app.allowRendererProcessReuse is deprecated, it is
// currently "false".  It will change to be "true" in Electron 9.  For more
// information please check https://github.com/electron/electron/issues/18397
app.allowRendererProcessReuse = true;

const home = app.getPath("home");
const identitiesPath = `${home}/.radicle/identities`;
const currentIdentitiesPath = `${identitiesPath}/current`;
const electronPath = `${currentIdentitiesPath}/electron`;

// Make sure "<home>/.radicle/identities" exists
if (!fs.existsSync(identitiesPath))
  fs.mkdirSync(identitiesPath, { recursive: true });
// If no "current" symlink exists, create it pointing to a new empty folder under identities
if (!fs.existsSync(currentIdentitiesPath)) {
  const newIdentityFolder = uuid.v4();
  fs.mkdirSync(`${identitiesPath}/${newIdentityFolder}`);
  fs.symlinkSync(
    `${identitiesPath}/${newIdentityFolder}`,
    currentIdentitiesPath
  );
}
// Make sure the "electron" folder exists & overwrite the paths for the electron data
if (!fs.existsSync(electronPath)) fs.mkdirSync(electronPath);
app.setPath("userData", electronPath);
app.setPath("appData", electronPath);

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
  proxyArgs: [],
  lineLimit: 500,
});

ipcMain.handle(RendererMessage.DIALOG_SHOWOPENDIALOG, async () => {
  const window = windowManager.window;
  if (window === null) {
    return;
  }

  const result = await dialog.showOpenDialog(window, {
    properties: ["openDirectory", "showHiddenFiles", "createDirectory"],
  });

  if (result.filePaths.length === 1) {
    return result.filePaths[0];
  } else {
    return "";
  }
});

ipcMain.handle(RendererMessage.CLIPBOARD_WRITETEXT, async (_event, text) => {
  clipboard.writeText(text);
});

ipcMain.handle(RendererMessage.OPEN_PATH, async (_event, path) => {
  shell.openPath(path);
});

ipcMain.handle(RendererMessage.GET_VERSION, () => {
  return app.getVersion();
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
  windowManager.open();
});
