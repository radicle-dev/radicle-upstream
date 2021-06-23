// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { ipcRenderer, contextBridge } = require("electron");

contextBridge.exposeInMainWorld("electron", {
  ipcRenderer: {
    invoke: ipcRenderer.invoke.bind(ipcRenderer),
    on: ipcRenderer.on.bind(ipcRenderer),
  },
  isDev: process.env.NODE_ENV === "development",
  isExperimental: process.env.RADICLE_UPSTREAM_EXPERIMENTAL === "true",
});
