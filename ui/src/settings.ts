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

export const defaultSetttings = (): Settings => ({
  appearance: {
    theme: Theme.Light,
    hints: {
      showRemoteHelper: true,
    },
  },
  coco: {
    seeds: [],
  },
});

// gives back the OS you're using in hotkeys.svelte & shortcuts.svelte
export const isMac: boolean = navigator.platform.includes("Mac");
