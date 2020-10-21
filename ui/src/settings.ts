import { get } from "svelte/store";

import { createValidationStore, ValidationStatus } from "./validation";
import * as notification from "./notification";
import { settings, updateCoCo } from "./session";

// TYPES
export enum Theme {
  Dark = "dark",
  Light = "light",
}

export interface Appearance {
  theme: Theme;
  hints: {
    showRemoteHelper: boolean;
  };
}

export interface CoCo {
  seeds: string[];
}

export interface Settings {
  appearance: Appearance;
  coco: CoCo;
}

interface Option {
  value: string;
  title: string;
}

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

const VALID_SEED_MATCH = /^[\w\d]{54}@[\w\d.]*:[\d]{1,5}$/;

const checkSeedUniqueness = (seed: string): Promise<boolean> => {
  return Promise.resolve(!get(settings).coco.seeds.includes(seed));
};

export const seedValidation = createValidationStore(
  {
    format: {
      pattern: VALID_SEED_MATCH,
      message: "This is not a valid seed address",
    },
  },
  [
    {
      promise: checkSeedUniqueness,
      validationMessage: "This seed already exists",
    },
  ]
);

let restartNotificationShown = false;

const showRestartNotification = (): void => {
  // The restart notification is sticky and will stay visible until the user
  // clicks the "Restart" button, no need to show it multiple times.
  if (restartNotificationShown) return;

  notification.error(
    "Restart the app to connect to your new seed",
    false,
    "Restart",
    () => {
      // TODO(rudolfs): implement proxy/app restart
      console.log("restart");
    },
    true
  );

  restartNotificationShown = true;
};

export const addSeed = async (seed: string): Promise<boolean> => {
  // This has to be awaited contrary to what tslint suggests, because we're
  // running async remote validations in in the background. If we remove the
  // async then the seed input form will have to be submitted twice to take any
  // effect.
  await seedValidation.validate(seed);
  if (get(seedValidation).status !== ValidationStatus.Success) return false;

  updateCoCo({ seeds: [...get(settings).coco.seeds, seed] });
  showRestartNotification();
  return true;
};

export const removeSeed = (seed: string): void => {
  updateCoCo({
    seeds: get(settings).coco.seeds.filter((x: string) => x !== seed),
  });
  showRestartNotification();
};
