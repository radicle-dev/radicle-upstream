<script>
  import { location, pop, push } from "svelte-spa-router";

  import * as modal from "./src/modal.ts";
  import * as path from "./src/path.ts";
  import { isMac } from "./src/settings.ts";

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

    if (event.target !== document.body || event.repeat) {
      return false;
    }

    // To open help => ?
    if (event.key === "?") {
      toggle(path.shortcuts());
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
    if (modifierKey && event.key === "d") {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (modifierKey && event.key === "n") {
      toggle(path.createProject());
    }

    if (event.code === "Escape") {
      modal.hide();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
