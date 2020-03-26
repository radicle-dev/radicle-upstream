import { IPC_CLIPBOARD_WRITETEXT, IPC_DIALOG_SHOWOPENDIALOG } from "./types.js";

// We have to be able to select empty directories when we create new
// projects. Unfortunately we can't use the HTML5 open dialog via
// <input type="file"> for this. Although it lets us select directories,
// it doesn't fire an event when an empty directory is selected.
//
// The workaround is to use the electron native open dialog. As a bonus we
// can configure it to allow users to create new directories.
export const getDirectoryPath = () =>
  window.electron.ipcRenderer.invoke(IPC_DIALOG_SHOWOPENDIALOG);

export const copyToClipboard = text =>
  window.electron.ipcRenderer.invoke(IPC_CLIPBOARD_WRITETEXT, text);
