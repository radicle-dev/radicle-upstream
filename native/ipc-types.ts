// Messages sent from the main process to the renderer
export type MainMessage = {
  kind: MainMessageKind.PROXY_ERROR;
  data: ProxyError;
};

export enum MainMessageKind {
  PROXY_ERROR = "PROXY_ERROR",
}

// Payload for the ProxyError `MainMessage`.
export interface ProxyError {
  status: number | null;
  signal: NodeJS.Signals | null;
  output: string;
}

// RPC interface exposed by the main process to the renderer.
export interface MainProcess {
  clipboardWriteText(text: string): Promise<void>;
  getVersion(): Promise<string>;
  openPath(path: string): Promise<void>;
  openUrl(path: string): Promise<void>;
  // Open a system dialog to select a directory and returns the
  // selected directory.
  selectDirectory(): Promise<string>;
  // Get the git global default branch, which can be customized by the user.
  getGitGlobalDefaultBranch(): Promise<string | undefined>;
}

export const mainProcessMethods: Array<keyof MainProcess> = [
  "clipboardWriteText",
  "getVersion",
  "openPath",
  "openUrl",
  "selectDirectory",
  "getGitGlobalDefaultBranch",
];
