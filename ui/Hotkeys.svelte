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

  const onKeydown = event => {
    const modifierKey = isMac ? event.metaKey : event.ctrlKey;

    if (event.target !== document.body) {
      return false;
    }

    // To open help => OS modifier key + /
    if (event.shiftKey && event.code === "Slash") {
      toggle(path.shortcuts());
    }

    // To open settings => OS modifier key + ,
    if (modifierKey && event.code === "Comma") {
      toggle(path.settings());
    }

    // To open search => OS modifier key + p
    if (modifierKey && event.code === "KeyP") {
      toggleModal(path.search());
    }

    // To open design system => OS modifier key + d
    if (modifierKey && event.code === "KeyD") {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (modifierKey && event.code === "KeyN") {
      toggle(path.createProject());
    }

    if (event.code === "Escape") {
      modal.hide();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
