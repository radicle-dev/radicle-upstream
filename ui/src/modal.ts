import { get, writable } from "svelte/store";
import { SvelteComponent } from "svelte";

import SearchModal from "../DesignSystem/Component/SearchModal.svelte";

type ModalOverlay =
  | { show: true; component: typeof SvelteComponent }
  | { show: false; component: null };

type ModalRoute = "/search";

const routes: Record<ModalRoute, typeof SvelteComponent> = {
  "/search": SearchModal,
};

export const store = writable<ModalOverlay>({ show: false, component: null });

export const hide = () => store.set({ show: false, component: null });

export const toggle = (path: ModalRoute): void => {
  // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access
  if (get(store).show) {
    hide();
    return;
  }

  // show
  const component = routes[path];
  store.set({ show: true, component });
};
