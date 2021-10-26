// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// We have to keep the exported TextInput types in a separate .ts file because
// we use them in other .ts files and tsc/webpack are not smart enough to
// pick up type definitions exported from `context="module"` sections in
// .svelte files yet.
//
// See https://github.com/sveltejs/svelte/issues/5817 for more details.

export type TextInputValidationState =
  | { type: "unvalidated" }
  | { type: "pending" }
  | { type: "valid" }
  | { type: "invalid"; message: string };
