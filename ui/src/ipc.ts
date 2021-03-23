// eslint-disable-next-line @typescript-eslint/triple-slash-reference,spaced-comment
/// <reference path="../../native/preload.d.ts" />
import * as ipcTypes from "../../native/ipc-types";

export type { ProxyError } from "../../native/ipc-types";

// `true` if we are running unit tests with Jest.
const isNodeTestEnv = Boolean(
  globalThis.process && globalThis.process.env["NODE_ENV"] === "test"
);

// `true` if this code is run by the Cypress test driver.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const isCypressTestEnv = Boolean((globalThis as any).cy);

function makeMainProcessClient(): ipcTypes.MainProcess {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const client: any = {};
  ipcTypes.mainProcessMethods.forEach(method => {
    client[method] = (arg: unknown) =>
      window.electron.ipcRenderer.invoke(method as unknown, arg);
  });
  return client;
}

const mainProcess = makeMainProcessClient();

export const getDirectoryPath = mainProcess.selectDirectory;

export const getVersion = mainProcess.getVersion;

export const copyToClipboard = mainProcess.clipboardWriteText;

export const openPath = mainProcess.openPath;

export const openUrl = mainProcess.openUrl;

export const getGitGlobalDefaultBranch = mainProcess.getGitGlobalDefaultBranch;

// Informs whether it's running in a development environment.
export const isDev = (): boolean => {
  return window.electron.isDev;
};

// Informs whether it's running in experimental mode, where
// features under construction are enabled and can thus be used.
// This option can only be enabled iff `isDev()` as we should only
// want to toggle it while in development mode.
export const isExperimental = (): boolean => {
  return window.electron.isExperimental;
};

// Register a listener for the `ipcTypes.ProxyError` message.
export function listenProxyError(
  f: (proxyError: ipcTypes.ProxyError) => void
): void {
  if (isNodeTestEnv || isCypressTestEnv) {
    return;
  }

  window.electron.ipcRenderer.on(
    "message",
    (_event: unknown, message: ipcTypes.MainMessage) => {
      if (message.kind === ipcTypes.MainMessageKind.PROXY_ERROR) {
        f(message.data);
      }
    }
  );
}
