import * as theme from "./theme";

export interface Appearance {
  theme: theme.Theme;
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
