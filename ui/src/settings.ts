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

export interface Settings {
  appearance: Appearance;
  registry: Registry;
}

interface Option {
  value: string;
  title: string;
}

export  const networkOptions: Option[] = [
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
