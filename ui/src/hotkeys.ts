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

export const keyboardShortcuts = [
  { title: "Keyboard shortcuts", key: ShortcutKey.Help },
  { title: "New project", key: ShortcutKey.NewProjects, modifierKey: true },
  { title: "Search", key: ShortcutKey.Search, modifierKey: true },
  { title: "Settings", key: ShortcutKey.Settings, modifierKey: true },
];

export const devShortcuts = [
  {
    title: "Design system",
    key: ShortcutKey.DesignSystem,
    modifierKey: true,
  },
];

export const OSModifierKey = isMac ? "⌘" : "ctrl";
