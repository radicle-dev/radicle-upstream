// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

export interface Plugin {
  start: () => Promise<null>;
  stop: () => Promise<null>;
}

export const methods: Array<keyof Plugin> = ["start", "stop"];
