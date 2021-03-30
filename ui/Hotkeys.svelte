<script lang="typescript">
  import { location, pop, push } from "svelte-spa-router";

  import * as modal from "./src/modal";
  import * as path from "./src/path";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "./src/config";

  const toggle = (destination: string) => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
    modal.hide();
  };

  const toggleModal = (destination: string) => {
    if (path.active(path.designSystemGuide(), $location)) {
      pop();
    }
    modal.toggle(destination);
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
        toggleModal(path.shortcuts());
        break;
      case hotkeys.ShortcutKey.Settings:
        toggle(path.settings());
        break;
      case hotkeys.ShortcutKey.Search:
        toggleModal(path.search());
        break;
      case hotkeys.ShortcutKey.DesignSystem:
        toggle(path.designSystemGuide());
        break;
      case hotkeys.ShortcutKey.NewProjects:
        toggleModal(path.newProject());
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
