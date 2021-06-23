// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

let lockRequests = 0;

export const isLocked = (): boolean => {
  return lockRequests > 0;
};

// Shows a spinning cursor and ignore user clicks while `f` is running.
export function withLock<T>(f: () => Promise<T>): Promise<T> {
  if (lockRequests === 0) {
    document.documentElement.classList.add("lock-screen");
  }
  lockRequests += 1;
  return f().finally(() => {
    lockRequests -= 1;
    if (lockRequests === 0) {
      document.documentElement.classList.remove("lock-screen");
    }
  });
}
