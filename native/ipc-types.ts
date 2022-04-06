// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Messages sent from the main process to the renderer
export type MainMessage =
  | {
      kind: MainMessageKind.PROXY_ERROR;
      data: ProxyError;
    }
  | {
      kind: MainMessageKind.CUSTOM_PROTOCOL_INVOCATION;
      data: CustomProtocolInvocation;
    };

export enum MainMessageKind {
  PROXY_ERROR = "PROXY_ERROR",
  CUSTOM_PROTOCOL_INVOCATION = "CUSTOM_PROTOCOL_INVOCATION",
}

// Payload for the ProxyError `MainMessage`.
export interface ProxyError {
  status: number | null;
  signal: NodeJS.Signals | null;
  output: string;
}

// Payload for the CustomProtocolInvocation `MainMessage`
export interface CustomProtocolInvocation {
  url: string;
}

// RPC interface exposed by the main process to the renderer.
export interface MainProcess {
  clipboardWriteText(text: string): Promise<void>;
  getVersion(): Promise<string>;
  getProxyLogs(): Promise<string>;
  openPath(path: string): Promise<void>;
  openUrl(path: string): Promise<void>;
  // Get the git global default branch, which can be customized by the user.
  getGitGlobalDefaultBranch(): Promise<string | undefined>;
  checkRadCliVersion(): Promise<string | undefined>;
  checkGitVersion(): Promise<string | undefined>;
}

export const mainProcessMethods: Array<keyof MainProcess> = [
  "clipboardWriteText",
  "getVersion",
  "getProxyLogs",
  "openPath",
  "openUrl",
  "getGitGlobalDefaultBranch",
  "checkGitVersion",
  "checkRadCliVersion",
];
