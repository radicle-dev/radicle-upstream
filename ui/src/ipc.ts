import type {} from "../../native/preload";
import * as ipcTypes from "../../native/ipc-types";
import * as config from "./config";

export type {
  ProxyError,
  CustomProtocolInvocation,
} from "../../native/ipc-types";

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

// Register a listener for the `ipcTypes.ProxyError` message.
export function listenProxyError(
  f: (proxyError: ipcTypes.ProxyError) => void
): void {
  if (config.isNodeTestEnv || config.isCypressTestEnv) {
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

// Register a listener for the `ipcTypes.CustomProtocolInvocation` message.
export function listenCustomProtocolInvocation(
  f: (customProtocolInvocation: ipcTypes.CustomProtocolInvocation) => void
): void {
  if (config.isNodeTestEnv || config.isCypressTestEnv) {
    return;
  }

  window.electron.ipcRenderer.on(
    "message",
    (_event: unknown, message: ipcTypes.MainMessage) => {
      if (
        message.kind === ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION
      ) {
        f(message.data);
      }
    }
  );
}
