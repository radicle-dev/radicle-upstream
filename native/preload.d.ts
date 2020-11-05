interface Window {
  electron: {
    ipcRenderer: {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      invoke: (cmd: string, args?: unknown) => any;
    };
    isDev: boolean;
    isExperimental: boolean;
  };
}
