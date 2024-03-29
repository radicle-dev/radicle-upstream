// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type {} from "native/preload";

import * as config from "ui/src/config";
import * as Bacon from "ui/src/bacon";
import * as ipcTypes from "native/ipc-types";
export type { ProxyError, CustomProtocolInvocation } from "native/ipc-types";

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

export const getVersion = mainProcess.getVersion;

export const getProxyLogs = mainProcess.getProxyLogs;

export const copyToClipboard = mainProcess.clipboardWriteText;

export const openPath = mainProcess.openPath;

export const openUrl = mainProcess.openUrl;

export const checkGitVersion = mainProcess.checkGitVersion;

export const checkRadCliVersion = mainProcess.checkRadCliVersion;

export const getGitGlobalDefaultBranch = mainProcess.getGitGlobalDefaultBranch;

const mainMessages = ((): Bacon.EventStream<ipcTypes.MainMessage> => {
  if (config.isNodeTestEnv) {
    return Bacon.never();
  } else {
    return Bacon.fromEvent(
      window.electron.ipcRenderer,
      "message",
      (_event, message) => message
    );
  }
})();

export const proxyError: Bacon.EventStream<ipcTypes.ProxyError> =
  Bacon.filterMap(mainMessages, message => {
    if (message.kind === ipcTypes.MainMessageKind.PROXY_ERROR) {
      return message.data;
    } else {
      return undefined;
    }
  });

export const customProtocolInvocation: Bacon.EventStream<ipcTypes.CustomProtocolInvocation> =
  Bacon.filterMap(mainMessages, message => {
    if (message.kind === ipcTypes.MainMessageKind.CUSTOM_PROTOCOL_INVOCATION) {
      return message.data;
    } else {
      return undefined;
    }
  });
