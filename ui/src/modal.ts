import { derived, get, writable, Readable } from "svelte/store";

type ModalOverlay =
  | { show: true; route: string }
  | { show: false; route: null };

const overlayStore = writable<ModalOverlay>({
  show: false,
  route: null,
});
export const store = derived(overlayStore, $store => $store);

export const hide = (): void => overlayStore.set({ show: false, route: null });

export const toggle = (route: string): void => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  if (get(store).show && get(store).route === route) {
    hide();
    return;
  }

  overlayStore.set({ show: true, route });
};

export const showing: Readable<boolean> = derived(store, $store => $store.show);
