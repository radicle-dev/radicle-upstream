<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { pop, push, activeRouteStore } from "ui/src/router";
  import type { SvelteComponent } from "svelte";

  import SearchModal from "ui/App/SearchModal.svelte";
  import ShortcutsModal from "ui/App/ShortcutsModal.svelte";

  import * as modal from "ui/src/modal";
  import * as screen from "ui/src/screen";
  import * as hotkeys from "ui/src/hotkeys";
  import * as router from "ui/src/router";
  import { config } from "ui/src/config";

  const toggleModal = (modalComponent: typeof SvelteComponent) => {
    if ($activeRouteStore.type === "designSystemGuide") {
      pop();
    }
    modal.toggle(modalComponent);
  };

  const onKeydown = (event: KeyboardEvent) => {
    const modifierKey = hotkeys.isMac ? event.metaKey : event.ctrlKey;
    const hasInputTarget =
      !modifierKey &&
      event.target &&
      (event.target as HTMLInputElement).type === "text";

    if (
      !hotkeys.areEnabled() ||
      screen.isLocked() ||
      hasInputTarget ||
      event.repeat ||
      event.altKey
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
      case hotkeys.ShortcutKey.GoBack:
        router.pop();
        break;
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
      case hotkeys.ShortcutKey.Network:
        modal.hide();
        if ($activeRouteStore.type === "network") {
          return;
        }
        push({ type: "network" });
        break;
      case hotkeys.ShortcutKey.Diagnostics:
        modal.hide();
        if ($activeRouteStore.type === "diagnostics") {
          return;
        }
        push({ type: "diagnostics", activeTab: "storage" });
        break;
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
