%raw
{|
  const electron = require('electron');

  const app = electron.app;
  const BrowserWindow = electron.BrowserWindow;
  const path = require('path');
  const isDev = require('electron-is-dev');

  let mainWindow;
  let proxyChildProcess;

  async function startApp() {
    await startProxy();
    createWindow();
  }

  function createWindow() {
    mainWindow = new BrowserWindow({width: 900, height: 680});
    mainWindow.loadURL(
      isDev
        ? 'http://localhost:8000'
        : `file://${path.join(__dirname, '../../../../build/index.html')}`,
    );
    mainWindow.on('closed', () => (mainWindow = null));
  }

  function startProxy() {
    const proxyPath = path.join(__dirname, '../../../../../proxy');
    const { execFile } = require('child_process');
    proxyChildProcess = execFile(proxyPath, [], (error, stdout, stderr) => {
      if (error) {
        console.log(error);
      }
      console.log(stdout);
    });

    console.log(proxyChildProcess);
  }

  app.on('ready', startApp);

  app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
      app.quit();
    }
  });

  app.on('will-quit', () => {
    proxyChildProcess.kill('SIGHUP');
  })

  app.on('activate', () => {
    if (mainWindow === null) {
      createWindow();
    }
  });
|};
