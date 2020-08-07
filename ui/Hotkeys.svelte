<script>
  import { location, pop, push } from "svelte-spa-router";

  import * as path from "./src/path.ts";

  const toggle = destination => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
  };

  const onKeydown = event => {
    const modifierKey = navigator.platform.includes("Mac")
      ? event.metaKey
      : event.ctrlKey;

    if (event.target !== document.body) {
      return false;
    }

    // To open help => OS modifier key + /
    if (modifierKey && event.code === "Slash") {
      toggle(path.help());
    }

    // To open settings => OS modifier key + ,
    if (modifierKey && event.code === "Comma") {
      toggle(path.settings());
    }

    // To open search => OS modifier key + p
    if (modifierKey && event.code === "KeyP") {
      toggle(path.search());
    }

    // To open design system => OS modifier key + d
    if (modifierKey && event.code === "KeyD") {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (modifierKey && event.code === "KeyN") {
      toggle(path.createProject());
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
