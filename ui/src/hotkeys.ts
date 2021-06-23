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
  Search = "p",
  Settings = ",",
  NetworkDiagnostics = "1",
}

export interface KeyboardShortcut {
  description: string;
  key: ShortcutKey;
  displayKey?: string; // if this is not the same as the encoded key value
  modifierKey?: boolean;
}

export const shortcuts: KeyboardShortcut[] = [
  { description: "Keyboard shortcuts", key: ShortcutKey.Help },
  {
    description: "New project",
    key: ShortcutKey.NewProjects,
    modifierKey: true,
  },
  { description: "Search", key: ShortcutKey.Search, modifierKey: true },
  { description: "Settings", key: ShortcutKey.Settings, modifierKey: true },
  {
    description: "Network diagnostics",
    key: ShortcutKey.NetworkDiagnostics,
    modifierKey: true,
  },
];

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
