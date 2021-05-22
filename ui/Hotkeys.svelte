<script lang="typescript">
  import { pop, push, routeStore } from "ui/src/router";
  import type { SvelteComponent } from "svelte";

  import NewProjectModal from "ui/Modal/NewProject.svelte";
  import SearchModal from "ui/Modal/Search.svelte";
  import ShortcutsModal from "ui/Modal/Shortcuts.svelte";

  import * as modal from "./src/modal";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "./src/config";

  const toggleModal = (modalComponent: typeof SvelteComponent) => {
    if ($routeStore.type === "designSystemGuide") {
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
        toggleModal(ShortcutsModal);
        break;
      case hotkeys.ShortcutKey.Settings:
        modal.hide();
        if ($routeStore.type === "settings") {
          return;
        }
        push({ type: "settings" });
        break;
      case hotkeys.ShortcutKey.Search:
        toggleModal(SearchModal);
        break;
      case hotkeys.ShortcutKey.DesignSystem:
        if ($routeStore.type === "designSystemGuide") {
          pop();
        } else {
          push({ type: "designSystemGuide" });
          modal.hide();
        }
        break;
      case hotkeys.ShortcutKey.NewProjects:
        toggleModal(NewProjectModal);
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
