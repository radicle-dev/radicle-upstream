<script lang="typescript">
  import { pop, push, state } from "ui/src/router";
  import type { SvelteComponent } from "svelte";

  import NewProjectModal from "ui/Modal/NewProject.svelte";
  import SearchModal from "ui/Modal/Search.svelte";
  import ShortcutsModal from "ui/Modal/Shortcuts.svelte";

  import SettingsScreen from "ui/Screen/Settings.svelte";
  import DesignSystemGuideScreen from "ui/Screen/DesignSystemGuide.svelte";

  import * as modal from "./src/modal";
  import * as path from "./src/path";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "./src/config";

  const show = (destination: typeof SvelteComponent) => {
    modal.hide();
    if (destination === $state.component) {
      return;
    }
    push({ component: destination });
  };

  const toggle = (destination: typeof SvelteComponent) => {
    if (destination === $state.component) {
      pop();
    }
    push({ component: destination });
    modal.hide();
  };

  const toggleModal = (modalComponent: typeof SvelteComponent) => {
    if (path.designSystemGuide() === $state.component) {
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
        show(SettingsScreen);
        break;
      case hotkeys.ShortcutKey.Search:
        toggleModal(SearchModal);
        break;
      case hotkeys.ShortcutKey.DesignSystem:
        toggle(DesignSystemGuideScreen);
        break;
      case hotkeys.ShortcutKey.NewProjects:
        toggleModal(NewProjectModal);
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
