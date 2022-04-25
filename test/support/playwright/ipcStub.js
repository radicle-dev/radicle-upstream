// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

const electronStubs = {
  getVersion: () => Promise.resolve("v1.2.3"),
  getProxyLogs: () => Promise.resolve("Dummy log line"),
  copyToClipboard: () => Promise.resolve(),
  openPath: () => Promise.resolve(),
  openUrl: () => Promise.resolve(),
  checkGitVersion: () => Promise.resolve("2.35.1"),
  checkRadCliVersion: () => Promise.resolve("0.4.0"),
  getGitGlobalDefaultBranch: () => Promise.resolve("trunk"),
};

window.electron = {
  ipcRenderer: {
    invoke: (msg, params) => {
      return electronStubs[msg](params);
    },
    on: (_event, _handle) => {},
  },
};
