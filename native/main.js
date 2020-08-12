import {
  app,
  BrowserWindow,
  ipcMain,
  dialog,
  clipboard,
  shell,
} from "electron";
import path from "path";
import * as ipc from "./ipc.js";

const isDev = process.env.NODE_ENV === "development";

// The default value of app.allowRendererProcessReuse is deprecated, it is
// currently "false".  It will change to be "true" in Electron 9.  For more
// information please check https://github.com/electron/electron/issues/18397
app.allowRendererProcessReuse = true;

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow;
let proxyChildProcess;

const startApp = () => {
  startProxy();
  createWindow();
};

ipcMain.handle(ipc.DIALOG_SHOWOPENDIALOG, async () => {
  const result = await dialog.showOpenDialog(mainWindow, {
    properties: ["openDirectory", "showHiddenFiles", "createDirectory"],
  });

  if (result.filePaths.length === 1) {
    return result.filePaths[0];
  } else {
    return "";
  }
});

ipcMain.handle(ipc.CLIPBOARD_WRITETEXT, async (_event, text) => {
  clipboard.writeText(text);
});

ipcMain.handle(ipc.OPEN_PATH, async (_event, path) => {
  shell.openPath(path);
});

const createWindow = () => {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 680,
    icon: path.join(__dirname, "../public/icon.png"),
    show: false,
    autoHideMenuBar: true,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
    },
  });

  mainWindow.once("ready-to-show", () => {
    mainWindow.maximize();
    mainWindow.show();
  });

  let watcher;
  if (isDev) {
    watcher = require("chokidar").watch(path.join(__dirname, "../public/**"), {
      ignoreInitial: true,
    });

    watcher.on("change", _p => {
      mainWindow.reload();
    });
  }

  mainWindow.webContents.on("will-navigate", (event, url) => {
    event.preventDefault();
    if (
      url.toLowerCase().startsWith("http://") ||
      url.toLowerCase().startsWith("https://")
    ) {
      console.log(`Opening external URL: ${url}`);
      shell.openExternal(url);
    } else {
      console.warn(`User tried opening URL with invalid URI scheme: ${url}`);
    }
  });

  mainWindow.loadURL(`file://${path.join(__dirname, "../public/index.html")}`);
  mainWindow.on("closed", () => {
    if (watcher) {
      watcher.close();
    }

    mainWindow = null;
  });
};

const startProxy = () => {
  if (isDev) {
    return;
  }

  const proxyPath = path.join(__dirname, "../../proxy");
  const { execFile } = require("child_process");
  proxyChildProcess = execFile(
    proxyPath,
    ["--registry=emulator"],
    (error, _stdout, _stderr) => {
      if (error) {
        console.log(error);
      }
    }
  );
};

app.on("will-quit", () => {
  if (proxyChildProcess) {
    proxyChildProcess.kill("SIGHUP");
  }
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", startApp);

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) {
    createWindow();
  }
});
