import type { MainMessage } from "./ipc-types";

declare global {
  interface Window {
    electron: {
      ipcRenderer: {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        invoke: (cmd: unknown, args?: unknown) => Promise<any>;
        on: (
          event: "message",
          handle: (event: unknown, message: MainMessage) => void
        ) => void;
      };
      isDev: boolean;
      isExperimental: boolean;
    };
  }
}
