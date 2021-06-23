// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { derived, get, writable } from "svelte/store";
import type { SvelteComponent } from "svelte";

type OnHide = () => void;
const doNothing = () => {};

type ModalOverlay = {
  modalComponent: typeof SvelteComponent;
  onHide: OnHide;
  modalComponentProps: unknown;
};

const overlayStore = writable<ModalOverlay | null>(null);
export const store = derived(overlayStore, $store => $store);

export const hide = (): void => {
  const stored = get(store);
  if (stored === null) {
    return;
  }

  stored.onHide();
  overlayStore.set(null);
};

export const show = (
  modalComponent: typeof SvelteComponent,
  onHide: OnHide = doNothing,
  modalComponentProps: unknown = {}
): void => {
  overlayStore.set({ modalComponent, onHide, modalComponentProps });
};

export const toggle = (
  modalComponent: typeof SvelteComponent,
  onHide: OnHide = doNothing,
  modalComponentProps: unknown = {}
): void => {
  const stored = get(store);

  if (stored && stored.modalComponent === modalComponent) {
    hide();
    return;
  }

  show(modalComponent, onHide, modalComponentProps);
};
