<script lang="typescript">
  import { location, pop, push } from "ui/src/router";
  import type { SvelteComponent } from "svelte";

  import ModalNewProject from "./Modal/NewProject.svelte";
  import ModalSearch from "./Modal/Search.svelte";
  import ModalShortcuts from "./Modal/Shortcuts.svelte";

  import * as modal from "./src/modal";
  import * as path from "./src/path";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "./src/config";

  const show = (destination: string) => {
    modal.hide();
    if (destination === $location) {
      return;
    }
    push(destination);
  };

  const toggle = (destination: string) => {
    if (destination === $location) {
      pop();
    }
    push(destination);
    modal.hide();
  };

  const toggleModal = (modalComponent: typeof SvelteComponent) => {
    if (path.designSystemGuide() === $location) {
      pop();
    }
    modal.toggle(modalComponent);
  };

  const onKeydown = (event: KeyboardEvent) => {
    const modifierKey = isMac ? event.metaKey : event.ctrlKey;
    const hasInputTarget =
      !modifierKey &&
      event.target &&
      (event.target as HTMLInputElement).type === "text";

    if (
      !hotkeys.areEnabled() ||
      screen.isLocked() ||
      hasInputTarget ||
      event.repeat
    ) {
      return false;
    }

    if (event.key === hotkeys.escape.key) {
      modal.hide();
      return;
    }

    const shortcut = [
      ...hotkeys.shortcuts,
      ...(isDev ? hotkeys.devShortcuts : []),
    ].find(shortcut => {
      const match = shortcut.key === event.key;
      return shortcut.modifierKey ? match && modifierKey : match;
    });

    if (!shortcut) return;

    switch (shortcut.key) {
      case hotkeys.ShortcutKey.Help:
        toggleModal(ModalShortcuts);
        break;
      case hotkeys.ShortcutKey.Settings:
        show(path.settings());
        break;
      case hotkeys.ShortcutKey.Search:
        toggleModal(ModalSearch);
        break;
      case hotkeys.ShortcutKey.DesignSystem:
        toggle(path.designSystemGuide());
        break;
      case hotkeys.ShortcutKey.NewProjects:
        toggleModal(ModalNewProject);
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
