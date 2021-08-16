// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

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
    };
  }
}
