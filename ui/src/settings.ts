// TYPES
export enum Theme {
  Dark = "dark",
  Light = "light",
}

export interface Appearance {
  theme: Theme;
}

export enum Network {
  Emulator = "emulator",
  FFnet = "ffnet",
  Testnet = "testnet",
}

export interface Registry {
  network: Network;
}

export interface CoCo {
  seeds: string[];
}

export interface Settings {
  appearance: Appearance;
  registry: Registry;
  coco: CoCo;
}

interface Option {
  value: string;
  title: string;
}

export const networkOptions: Option[] = [
  {
    value: Network.Emulator,
    title: "Emulator",
  },
  {
    value: Network.FFnet,
    title: "FFnet",
  },
  {
    value: Network.Testnet,
    title: "Testnet",
  },
];

export const themeOptions: Option[] = [
  {
    title: "Light",
    value: Theme.Light,
  },
  {
    title: "Dark",
    value: Theme.Dark,
  },
];

// gives back the OS you're using in hotkeys.svelte & shortcuts.svelte

export const isMac: boolean = navigator.platform.includes("Mac");
