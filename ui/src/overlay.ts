// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { derived, writable } from "svelte/store";

const currentStore = writable<HTMLDivElement | undefined>(undefined);
export const current = derived(currentStore, store => store);

export const open = (component: HTMLDivElement): void => {
  currentStore.set(component);
};

export const close = (): void => {
  currentStore.set(undefined);
};
