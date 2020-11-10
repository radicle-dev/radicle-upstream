import {
  app,
  BrowserWindow,
  ipcMain,
  dialog,
  clipboard,
  shell,
} from "electron";
import path from "path";
import { execFile, ChildProcess } from "child_process";
import { RendererMessage, MainMessage, MainMessageKind } from "./ipc-types";

const isDev = process.env.NODE_ENV === "development";
const proxyPath = path.join(__dirname, "../proxy/target/debug/api");

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
let proxyProcess: ChildProcess | undefined;

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
    url.toLowerCase().startsWith("irc://") ||
    url.toLowerCase().startsWith("http://") ||
    url.toLowerCase().startsWith("https://")
  ) {
    console.log(`Opening external URL: ${url}`);
    shell.openExternal(url);
  } else {
    console.warn(`User tried opening URL with invalid URI scheme: ${url}`);
  }
};

const startProxy = () => {
  if (isDev) {
    return;
  }

  proxyProcess = execFile(proxyPath, [], (error, stdout, stderr) => {
    let status = null;
    let signal = null;
    if (error) {
      status = error.code === undefined ? null : error.code;
      signal = error.signal === undefined ? null : error.signal;
    } else {
      status = 0;
    }

    console.error(
      "Proxy process exited with status code %s and signal %s",
      status,
      signal
    );

    windowManager.sendMessage({
      kind: MainMessageKind.PROXY_ERROR,
      data: {
        status,
        signal,
        stdout,
        stderr,
      },
    });
  });
};

app.on("will-quit", () => {
  if (proxyProcess) {
    proxyProcess.kill();
  }
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", () => {
  startProxy();

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
