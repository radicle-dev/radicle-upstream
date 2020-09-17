<script>
  import { location, pop, push } from "svelte-spa-router";

  import * as modal from "./src/modal.ts";
  import * as path from "./src/path.ts";
  import * as screen from "./src/screen.ts";
  import { isMac } from "./src/settings.ts";
  import * as hotkeys from "./src/hotkeys.ts";
  import { isDev } from "../native/ipc.js";

  const toggle = destination => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
  };

  const toggleModal = destination => modal.toggle(destination);

  // Don’t forget to update `ui/Screen/Shortcuts.svelte` if you update the key
  // bindings.
  const onKeydown = event => {
    const modifierKey = isMac ? event.metaKey : event.ctrlKey;

    if (
      !hotkeys.areEnabled() ||
      screen.isLocked() ||
      (!modifierKey && event.target.type === "text") ||
      event.repeat
    ) {
      return false;
    }

    // To open help => ?
    if (event.key === "?") {
      toggleModal(path.shortcuts());
    }

    // To open settings => OS modifier key + ,
    if (modifierKey && event.key === ",") {
      toggle(path.settings());
    }

    // To open search => OS modifier key + p
    if (modifierKey && event.key === "p") {
      toggleModal(path.search());
    }

    // To open design system => OS modifier key + d
    if (isDev() && modifierKey && event.key === "d") {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (modifierKey && event.key === "n") {
      toggleModal(path.newProject());
    }

    if (event.key === "Escape") {
      modal.hide();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
