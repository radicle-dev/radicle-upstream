export const DIALOG_SHOWOPENDIALOG = "IPC_DIALOG_SHOWOPENDIALOG";
export const CLIPBOARD_WRITETEXT = "IPC_CLIPBOARD_WRITETEXT";
export const OPEN_PATH = "IPC_OPEN_PATH";

// We have to be able to select empty directories when we create new
// projects. Unfortunately we can't use the HTML5 open dialog via
// <input type="file"> for this. Although it lets us select directories,
// it doesn't fire an event when an empty directory is selected.
//
// The workaround is to use the electron native open dialog. As a bonus we
// can configure it to allow users to create new directories.
export const getDirectoryPath = () =>
  window.electron.ipcRenderer.invoke(DIALOG_SHOWOPENDIALOG);

export const copyToClipboard = text =>
  window.electron.ipcRenderer.invoke(CLIPBOARD_WRITETEXT, text);

export const openPath = path =>
  window.electron.ipcRenderer.invoke(OPEN_PATH, path);

export const isDev = () => {
  return window.electron.isDev;
};
