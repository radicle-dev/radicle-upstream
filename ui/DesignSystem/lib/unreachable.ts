// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

export const unreachable = (value: never): never => {
  throw new Error(`Unreachable code: ${value}`);
};
