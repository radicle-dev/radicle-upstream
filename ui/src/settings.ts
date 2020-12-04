// TYPES
export interface Settings {
  appearance: Appearance;
  coco: CoCo;
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
