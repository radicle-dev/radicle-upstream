import { derived, writable } from "svelte/store";

enum Modal {
  IdentityCreation,
}

enum ScreenKind {
  Blank,
  Onboarding,
}

interface Blank {
  kind: ScreenKind.Blank;
}

export const screenBlank = (): Screen => ({
  kind: ScreenKind.Blank,
});

interface Onboarding {
  kind: ScreenKind.Onboarding;
}

export const screenOnboarding = (): Screen => ({
  kind: ScreenKind.Onboarding,
});

type Screen = Blank | Onboarding;

interface Item {
  modal?: Modal;
  screen: Screen;
}

const store = writable<Item>({ screen: { kind: ScreenKind.Blank } });
export const current = derived(store, store => store);

export const push = (screen: Screen, modal?: Modal): void => {
  store.set({ screen, modal });
};
