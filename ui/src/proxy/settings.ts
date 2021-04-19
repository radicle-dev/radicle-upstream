import * as zod from "zod";

export interface Settings {
  appearance: Appearance;
  coco: CoCo;
  featureFlags: FeatureFlags;
}

export interface FeatureFlags {
  funding: boolean;
}

export interface Appearance {
  codeFont: CodeFont;
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

export const settingsSchema: zod.ZodSchema<Settings> = zod.object({
  appearance: zod.object({
    codeFont: zod.enum([CodeFont.SourceCode, CodeFont.System]),
    hints: zod.object({
      showRemoteHelper: zod.boolean(),
    }),
    theme: zod.enum([Theme.Dark, Theme.Light, Theme.H4x0r]),
    uiFont: zod.enum([UIFont.Inter, UIFont.System]),
  }),
  coco: zod.object({
    seeds: zod.array(zod.string()),
  }),
  featureFlags: zod.object({
    funding: zod.boolean(),
  }),
});
