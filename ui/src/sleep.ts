// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

export function sleep(timeMs: number): Promise<void> {
  return new Promise(resolve => {
    setTimeout(resolve, timeMs);
  });
}
