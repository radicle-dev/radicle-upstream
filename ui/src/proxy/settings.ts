// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";

export interface Settings {
  appearance: Appearance;
  coco: CoCo;
}

export interface Appearance {
  codeFont: CodeFont;
  fontColor: FontColor;
  hints: {
    showRemoteHelper: boolean;
  };
  theme: Theme;
  uiFont: UIFont;
}

export interface CoCo {
  seeds: string[];
}

export enum Theme {
  Dark = "dark",
  Light = "light",
  H4x0r = "h4x0r",
}

export enum UIFont {
  Inter = "inter",
  System = "system",
}

export enum CodeFont {
  SourceCode = "sourceCode",
  System = "system",
}

export enum FontColor {
  Blue = "blue",
  Pink = "pink",
  Orange = "orange",
}

export const settingsSchema: zod.ZodSchema<Settings> = zod.object({
  appearance: zod.object({
    codeFont: zod.enum([CodeFont.SourceCode, CodeFont.System]),
    fontColor: zod.enum([FontColor.Blue, FontColor.Pink, FontColor.Orange]),
    hints: zod.object({
      showRemoteHelper: zod.boolean(),
    }),
    theme: zod.enum([Theme.Dark, Theme.Light, Theme.H4x0r]),
    uiFont: zod.enum([UIFont.Inter, UIFont.System]),
  }),
  coco: zod.object({
    seeds: zod.array(zod.string()),
  }),
});
