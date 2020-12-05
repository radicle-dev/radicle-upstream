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

// Message kinds sent from the renderer to the main process.
export enum RendererMessage {
  GET_VERSION = "GET_VERSION",
  OPEN_PATH = "IPC_OPEN_PATH",
}
