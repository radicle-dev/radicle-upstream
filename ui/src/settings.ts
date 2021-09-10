// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import {
  Appearance,
  CoCo,
  CodeFont,
  Settings,
  Theme,
  UIFont,
} from "./proxy/settings";

export type { Settings, Appearance, CoCo };
export { CodeFont, Theme, UIFont };

export const defaultSetttings = (): Settings => ({
  appearance: {
    theme: Theme.Dark,
    uiFont: UIFont.Inter,
    codeFont: CodeFont.SourceCode,
    hints: {
      showRemoteHelper: true,
    },
  },
  coco: {
    seeds: [],
  },
});

interface Option<T> {
  value: T;
  title: string;
}

export const themeOptions: Option<string>[] = [
  {
    title: "Light",
    value: Theme.Light,
  },
  {
    title: "Dark",
    value: Theme.Dark,
  },
  {
    title: "H4x0r",
    value: Theme.H4x0r,
  },
];

export const uiFontOptions: Option<string>[] = [
  {
    title: "Inter",
    value: UIFont.Inter,
  },
  {
    title: "System",
    value: UIFont.System,
  },
];

export const codeFontOptions: Option<string>[] = [
  {
    title: "Source Code",
    value: CodeFont.SourceCode,
  },
  {
    title: "System",
    value: CodeFont.System,
  },
];

// gives back the OS you're using in hotkeys.svelte & shortcuts.svelte
export const isMac: boolean = navigator.platform.includes("Mac");
