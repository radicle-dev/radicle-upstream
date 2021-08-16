<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { pop, push, activeRouteStore } from "ui/src/router";
  import type { SvelteComponent } from "svelte";

  import NewProjectModal from "ui/Modal/NewProject.svelte";
  import SearchModal from "ui/Modal/Search.svelte";
  import ShortcutsModal from "ui/Modal/Shortcuts.svelte";

  import * as modal from "./src/modal";
  import * as screen from "./src/screen";
  import { isMac } from "./src/settings";
  import * as hotkeys from "./src/hotkeys";
  import { config } from "./src/config";

  const toggleModal = (modalComponent: typeof SvelteComponent) => {
    if ($activeRouteStore.type === "designSystemGuide") {
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
      ...(config.isDev ? hotkeys.devShortcuts : []),
    ].find(shortcut => {
      const match = shortcut.key === event.key;
      return shortcut.modifierKey ? match && modifierKey : match;
    });

    if (!shortcut) {
      return;
    }

    switch (shortcut.key) {
      case hotkeys.ShortcutKey.Help:
        toggleModal(ShortcutsModal);
        break;
      case hotkeys.ShortcutKey.Settings:
        modal.hide();
        if ($activeRouteStore.type === "settings") {
          return;
        }
        push({ type: "settings" });
        break;
      case hotkeys.ShortcutKey.Search:
        toggleModal(SearchModal);
        break;
      case hotkeys.ShortcutKey.DesignSystem:
        if ($activeRouteStore.type === "designSystemGuide") {
          pop();
        } else {
          push({ type: "designSystemGuide" });
          modal.hide();
        }
        break;
      case hotkeys.ShortcutKey.NewProjects:
        toggleModal(NewProjectModal);
        break;
      case hotkeys.ShortcutKey.NetworkDiagnostics:
        modal.hide();
        if ($activeRouteStore.type === "networkDiagnostics") {
          return;
        }
        push({ type: "networkDiagnostics", activeTab: "peers" });
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
