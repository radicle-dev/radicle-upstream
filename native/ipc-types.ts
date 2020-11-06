// Messages send from the main process to the renderer
export type MainMessage = { type: "ProxyError"; data: ProxyError };

// Payload for the ProxyError `MainMessage`.
export interface ProxyError {
  status: number | null;
  signal: NodeJS.Signals | null;
  stdout: string;
  stderr: string;
}

// Message kinds send from the renderer to the main process.
export enum RendererMessage {
  CLIPBOARD_WRITETEXT = "IPC_CLIPBOARD_WRITETEXT",
  DIALOG_SHOWOPENDIALOG = "IPC_DIALOG_SHOWOPENDIALOG",
  GET_VERSION = "GET_VERSION",
  OPEN_PATH = "IPC_OPEN_PATH",
}
