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

type Theme = "dark" | "light" | "h4x0r";

export const theme = browserStore.create<Theme>(
  "radicle.appearance.theme",
  "dark",
  zod.enum(["dark", "light", "h4x0r"])
);

export const themeOptions: Option<Theme>[] = [
  {
    title: "Light",
    value: "light",
  },
  {
    title: "Dark",
    value: "dark",
  },
  {
    title: "H4x0r",
    value: "h4x0r",
  },
];

type UiFont = "inter" | "system";

export const uiFont = browserStore.create<UiFont>(
  "radicle.appearance.uiFont",
  "inter",
  zod.enum(["inter", "system"])
);

export const uiFontOptions: Option<UiFont>[] = [
  {
    title: "Inter",
    value: "inter",
  },
  {
    title: "System",
    value: "system",
  },
];

type CodeFont = "sourceCode" | "system";

export const codeFont = browserStore.create<CodeFont>(
  "radicle.appearance.codeFont",
  "sourceCode",
  zod.enum(["sourceCode", "system"])
);

export const codeFontOptions: Option<CodeFont>[] = [
  {
    title: "Source Code",
    value: "sourceCode",
  },
  {
    title: "System",
    value: "system",
  },
];

type PrimaryColor = "blue" | "pink" | "orange" | "custom";

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
  "#5555FF",
  zod.string()
);
