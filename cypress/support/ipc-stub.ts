import * as ipc from "../../native/ipc";
import * as sinon from "sinon";

// Stubs for Electron IPC message handlers.
//
// `ipcRenderer.invoke(msg, params)` uses the `msg` argument to look
// up the stub and call it.
interface ElectronStubs {
  [ipc.GET_VERSION]: sinon.SinonStub;
  [ipc.DIALOG_SHOWOPENDIALOG]: sinon.SinonStub;
  [ipc.OPEN_PATH]: sinon.SinonStub;
}

declare global {
  interface Window {
    electronStubs: ElectronStubs;
  }
}

// Stub the Electron API on the window object and add an
// `ElectronStubs` object that can be obtained with `getStubs()`.
//
// See `../../native/preload.js`.
export function setup(window: Window): void {
  const electronStubs: ElectronStubs = {
    [ipc.GET_VERSION]: sinon.stub().returns(Promise.resolve("v1.2.3")),
    [ipc.DIALOG_SHOWOPENDIALOG]: sinon
      .stub()
      .throws(new Error("not implemented")),
    [ipc.OPEN_PATH]: sinon.stub().throws(new Error("not implemented")),
  };

  window.electronStubs = electronStubs;

  window.electron = {
    ipcRenderer: {
      invoke: (msg, params) => {
        return electronStubs[msg](params);
      },
    },
    isDev: true,
    isExperimental: true,
  };
}

// Get the `ElectronStubs` object to stub and observe interactions of
// the app with the main process.
export function getStubs(): Cypress.Chainable<ElectronStubs> {
  return cy.window().then(w => w.electronStubs);
}
