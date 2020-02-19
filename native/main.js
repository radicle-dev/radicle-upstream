import { app, BrowserWindow, ipcMain, dialog } from "electron";
import path from "path";
import { MAIN_IPC_CHANNEL } from "../src/lib/types.js";

const isDev = process.env.NODE_ENV === "development";

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow;
let proxyChildProcess;

const startApp = () => {
  startProxy();
  createWindow();
};

ipcMain.handle(MAIN_IPC_CHANNEL, async () => {
  const result = await dialog.showOpenDialog(mainWindow, {
    properties: ["openDirectory", "showHiddenFiles", "createDirectory"]
  });

  if (result.filePaths.length === 1) {
    return result.filePaths[0];
  } else {
    return "";
  }
});

const createWindow = () => {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 680,
    icon: path.join(__dirname, "../public/icon.png"),
    show: false,
    webPreferences: {
      preload: path.join(__dirname, "preload.js")
    }
  });

  mainWindow.maximize();
  mainWindow.show();

  let watcher;
  if (isDev) {
    watcher = require("chokidar").watch(path.join(__dirname, "../public/**"), {
      ignoreInitial: true
    });

    watcher.on("change", _p => {
      mainWindow.reload();
    });
  }

  mainWindow.loadURL(
    `file://${path.join(__dirname, "../public/index.html#/projects")}`
  );
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
    ["--source=memory"],
    (error, stdout, _stderr) => {
      if (error) {
        console.log(error);
      }
      console.log(stdout);
    }
  );

  console.log(proxyChildProcess);
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
    startApp();
  }
});
