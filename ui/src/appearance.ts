// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";

import * as browserStore from "ui/src/browserStore";

interface Option<T> {
  value: T;
  title: string;
}

export enum Theme {
  Dark = "dark",
  Light = "light",
  H4x0r = "h4x0r",
}

export const theme = browserStore.create<Theme>(
  "radicle.appearance.theme",
  Theme.Dark,
  zod.enum([Theme.Dark, Theme.Light, Theme.H4x0r])
);

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

export enum UiFont {
  Inter = "inter",
  System = "system",
}

export const uiFont = browserStore.create<UiFont>(
  "radicle.appearance.uiFont",
  UiFont.Inter,
  zod.enum([UiFont.Inter, UiFont.System])
);

export const uiFontOptions: Option<UiFont>[] = [
  {
    title: "Inter",
    value: UiFont.Inter,
  },
  {
    title: "System",
    value: UiFont.System,
  },
];

export enum CodeFont {
  SourceCode = "sourceCode",
  System = "system",
}

export const codeFont = browserStore.create<CodeFont>(
  "radicle.appearance.codeFont",
  CodeFont.SourceCode,
  zod.enum([CodeFont.SourceCode, CodeFont.System])
);

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

export type PrimaryColor = "blue" | "pink" | "orange" | "custom";

export const primaryColor = browserStore.create<PrimaryColor>(
  "radicle.appearance.primaryColor",
  "blue",
  zod.enum(["blue", "pink", "orange", "custom"])
);

export const primaryColorOptions: Option<PrimaryColor>[] = [
  { title: "Blue", value: "blue" },
  { title: "Pink", value: "pink" },
  { title: "Orange", value: "orange" },
  { title: "Custom", value: "custom" },
];

export const primaryColorHex = browserStore.create<string>(
  "radicle.appearance.primaryColorHex",
  "#000000",
  zod.string()
);
