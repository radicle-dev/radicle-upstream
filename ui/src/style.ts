// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// Shared types and functionality related to styling

export function ellipsed(x: string, length: number = 8, offset: number = 2): string {
  return `${x.slice(0, length + offset)}…${x.slice(-length)}`;
}
