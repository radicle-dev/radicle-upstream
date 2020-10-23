import { derived, get, writable } from "svelte/store";

type ModalOverlay =
  | { show: true; route: string }
  | { show: false; route: null };

const overlayStore = writable<ModalOverlay>({
  show: false,
  route: null,
  context: null,
});
export const store = derived(overlayStore, $store => $store);

export const hide = (): void =>
  overlayStore.set({ show: false, route: null, contexts: null });

// TODO(rudolfs): what's the type of contexts?
export const toggle = (route: string, contexts: any): void => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  if (get(store).show && get(store).route === route) {
    hide();
    return;
  }

  overlayStore.set({ show: true, route, contexts });
};
