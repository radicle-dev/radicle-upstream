import { createValidationStore } from "./validation";

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

// TODO(sos): Do these validations in proxy; test them there
const VALID_SEED_MATCH_STR = "^[a-z0-9][a-z0-9.]+$";
const VALID_SEED_MATCH = new RegExp(VALID_SEED_MATCH_STR);

export const seedValidation = createValidationStore({
  presence: {
    message: "Seed cannot be empty",
  },
  format: {
    pattern: VALID_SEED_MATCH,
    message: "Seed may only contain letters, numbers, and dots",
  },
});
