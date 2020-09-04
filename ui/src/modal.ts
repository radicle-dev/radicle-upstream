import { get, writable } from "svelte/store";

type ModalOverlay =
  | { show: true; route: string }
  | { show: false; route: null };

export const store = writable<ModalOverlay>({ show: false, route: null });

export const hide = (): void => store.set({ show: false, route: null });

export const toggle = (route: string): void => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  if (get(store).show && get(store).route === route) {
    hide();
    return;
  }

  store.set({ show: true, route });
};
