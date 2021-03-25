import type {} from "../../native/preload";
import type { MainMessage, MainProcess } from "../../native/ipc-types";
import { EventEmitter } from "events";
import * as sinon from "sinon";

// Stubs for Electron IPC message handlers.
//
// `ipcRenderer.invoke(msg, params)` uses the `msg` argument to look
// up the stub and call it.
interface ElectronStubs extends MainProcess {
  getVersion: sinon.SinonStub;
  selectDirectory: sinon.SinonStub;
  openPath: sinon.SinonStub;
  openUrl: sinon.SinonStub;
  getGitGlobalDefaultBranch: sinon.SinonStub;
  sendMessage: (message: MainMessage) => void;
  getClipboard: () => string;
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
  const ipcRendererMessages = new EventEmitter();
  let clipboard = "";

  const electronStubs: ElectronStubs = {
    getVersion: sinon.stub().returns(Promise.resolve("v1.2.3")),
    selectDirectory: sinon.stub().throws(new Error("not implemented")),
    openPath: sinon.stub().throws(new Error("not implemented")),
    openUrl: sinon.stub(),
    async clipboardWriteText(text: string): Promise<void> {
      clipboard = text;
    },
    getGitGlobalDefaultBranch: sinon.stub().returns(Promise.resolve("trunk")),
    sendMessage: (message: MainMessage) => {
      ipcRendererMessages.emit("message", undefined, message);
    },
    getClipboard: () => clipboard,
  };

  window.electronStubs = electronStubs;

  window.electron = {
    ipcRenderer: {
      invoke: (msg, params) => {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        return (electronStubs as any)[msg as any](params as any);
      },
      on: ipcRendererMessages.on.bind(ipcRendererMessages),
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
