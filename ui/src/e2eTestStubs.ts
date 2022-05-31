// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { MainProcess, MainMessage } from "native/ipc-types";
import * as Sinon from "sinon";
import * as Bacon from "baconjs";
import * as FakeTimers from "@sinonjs/fake-timers";

import { config } from "ui/src/config";

declare global {
  interface Window {
    electronMainProcessStubs: typeof mainProcessStubs;
    fakeClock: FakeTimers.InstalledClock;
  }
}

const messageBus = new Bacon.Bus<MainMessage>();

const mainProcessStubs = {
  getVersion: () => Promise.resolve("v1.2.3"),
  getProxyLogs: () => Promise.resolve("Dummy log line"),
  openPath: Sinon.spy(() => Promise.resolve()),
  openUrl: Sinon.spy(() => Promise.resolve()),
  checkGitVersion: () => Promise.resolve("2.35.1"),
  checkRadCliVersion: () => Promise.resolve("0.4.0"),
  getGitGlobalDefaultBranch: () => Promise.resolve("trunk"),
  clipboardWriteText: Sinon.spy((_text: string) => Promise.resolve()),
  sendMessage(message: MainMessage) {
    messageBus.push(message);
  },
};

// Ensure that we implement the `MainProcess` interface with type
// asserations.
mainProcessStubs as MainProcess;

window.electronMainProcessStubs = mainProcessStubs;

window.electron = {
  ipcRenderer: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    invoke: async (cmd: any, args?: unknown) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return (mainProcessStubs as any)[cmd](args);
    },
    on: (event: string, handle) => {
      if (event !== "message") {
        throw new Error(`Invalid electron event ${event}`);
      }
      messageBus.onValue(message => handle(event, message));
    },
  },
};

if (config.fakeClock) {
  window.fakeClock = FakeTimers.install({ shouldClearNativeTimers: true });
}
