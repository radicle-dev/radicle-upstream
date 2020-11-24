<script lang="typescript">
  import { location, pop, push } from "svelte-spa-router";

  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "./src/ipc";
  import * as modal from "./src/modal";
  import * as path from "./src/path";
  import * as screen from "./src/screen";
  import { settings, updateAppearance } from "./src/session";
  import { isMac, toggleTheme } from "./src/settings";

  const toggle = (destination: string) => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
  };

  const toggleModal = (destination: string) => modal.toggle(destination);

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
      ...(isDev() ? hotkeys.devShortcuts : []),
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
      case hotkeys.ShortcutKey.ToggleTheme:
        updateAppearance({
          ...$settings.appearance,
          theme: toggleTheme($settings.appearance.theme),
        });
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
