// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { writable, Writable } from "svelte/store";

// The last claimed Ethereum address.
// The attestation process has been started, but it may not be completed.
// MUST be lowercase.
export const lastClaimed: Writable<string | undefined> = writable(undefined);
