import * as ethereum from "./ethereum";

// TYPES
export interface Settings {
  appearance: Appearance;
  coco: CoCo;
  featureFlags: FeatureFlags;
}

export const defaultSetttings = (): Settings => ({
  appearance: {
    theme: Theme.Dark,
    hints: {
      showRemoteHelper: true,
    },
  },
  coco: {
    seeds: [],
  },
  featureFlags: {
    funding: false,
  },
});

export interface Appearance {
  theme: Theme;
  hints: {
    showRemoteHelper: boolean;
  };
}

export enum Theme {
  Dark = "dark",
  Light = "light",
}

export interface CoCo {
  seeds: string[];
}

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
];

export interface FeatureFlags {
  funding: boolean;
}

export const featureFlagOptions: Option<boolean>[] = [
  {
    title: "Enabled",
    value: true,
  },
  {
    title: "Disabled",
    value: false,
  },
];

export const fundingEnvironmentOptions: Option<ethereum.Environment>[] = [
  {
    title: ethereum.Environment.Local.toString(),
    value: ethereum.Environment.Local,
  },
  {
    title: ethereum.Environment.Ropsten.toString(),
    value: ethereum.Environment.Ropsten,
  },
];

// gives back the OS you're using in hotkeys.svelte & shortcuts.svelte
export const isMac: boolean = navigator.platform.includes("Mac");
