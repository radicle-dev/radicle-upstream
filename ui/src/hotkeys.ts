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
