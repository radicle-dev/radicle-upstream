// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { get, writable } from "svelte/store";

import { isMac } from "./settings";

const state = writable(true);

export const areEnabled = (): boolean => {
  return get(state);
};

export const enable = (): void => {
  state.set(true);
};

export const disable = (): void => {
  state.set(false);
};

export enum ShortcutKey {
  DesignSystem = "d",
  Help = "?",
  Escape = "Escape",
  NewProjects = "n",
  Network = "b",
  Search = "p",
  Settings = ",",
  NetworkDiagnostics = "1",
}

export interface KeyboardShortcut {
  description: string;
  // Key code.
  key: ShortcutKey;
  // If set, this will be shown to the user in the ShortcutsModal instead of
  // the `key` code. E.g. the `Escape` keycode is presented to the user as
  // `esc`.
  displayKey?: string;
  modifierKey?: boolean;
  // If `true`, this hotkey won't be shown to the user in the ShortcutsModal,
  // the hotkey itself will still work however.
  hide?: boolean;
}

export const shortcuts: KeyboardShortcut[] = [
  { description: "Keyboard shortcuts", key: ShortcutKey.Help },
  {
    description: "New project",
    key: ShortcutKey.NewProjects,
    modifierKey: true,
  },
  { description: "Search", key: ShortcutKey.Search, modifierKey: true },
  { description: "Network", key: ShortcutKey.Network, modifierKey: true },
  { description: "Settings", key: ShortcutKey.Settings, modifierKey: true },
  {
    description: "Network diagnostics",
    key: ShortcutKey.NetworkDiagnostics,
    modifierKey: true,
    hide: true,
  },
];

// These are enabled in development mode, but not shown in the shortcuts modal.
export const devShortcuts: KeyboardShortcut[] = [
  {
    description: "Design system",
    key: ShortcutKey.DesignSystem,
    modifierKey: true,
  },
];

export const escape: KeyboardShortcut = {
  description: "Close modal",
  key: ShortcutKey.Escape,
  displayKey: "esc",
};

export const osModifierKey = isMac ? "⌘" : "ctrl";
