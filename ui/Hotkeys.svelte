<script lang="ts">
  import { location, pop, push } from "svelte-spa-router";

  import * as modal from "./src/modal";
  import * as path from "./src/path";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { isDev } from "../native/ipc.js";

  const toggle = (destination: string) => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
  };

  const toggleModal = (destination: string) => modal.toggle(destination);

  // Don’t forget to update `ui/Screen/Shortcuts.svelte` if you update the key
  // bindings.
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

    // To open help => ?
    if (event.key === hotkeys.ShortcutKey.Help) {
      toggleModal(path.shortcuts());
    }

    // To open settings => OS modifier key + ,
    if (modifierKey && event.key === hotkeys.ShortcutKey.Settings) {
      toggle(path.settings());
    }

    // To open search => OS modifier key + p
    if (modifierKey && event.key === hotkeys.ShortcutKey.Search) {
      toggleModal(path.search());
    }

    // To open design system => OS modifier key + d
    if (
      isDev() &&
      modifierKey &&
      event.key === hotkeys.ShortcutKey.DesignSystem
    ) {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (modifierKey && event.key === hotkeys.ShortcutKey.NewProjects) {
      toggleModal(path.newProject());
    }

    if (event.key === hotkeys.ShortcutKey.Escape) {
      modal.hide();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
