import * as ethereum from "./ethereum";

import {
  Settings,
  Theme,
  Appearance,
  CoCo,
  FeatureFlags,
} from "./proxy/settings";

export type { Settings, Appearance, CoCo, FeatureFlags };
export { Theme };

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
