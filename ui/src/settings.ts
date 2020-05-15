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
