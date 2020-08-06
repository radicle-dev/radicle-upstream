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
    const OSKey = navigator.platform.includes("Mac")
      ? event.metaKey
      : event.ctrlKey;

    if (event.target !== document.body) {
      return false;
    }

    // To open help => OS modifier key + /
    if (OSKey && event.code === "Slash") {
      toggle(path.help());
    }

    // To open settings => OS modifier key + ,
    if (OSKey && event.code === "Comma") {
      toggle(path.settings());
    }

    // To open search => OS modifier key + p
    if (OSKey && event.code === "KeyP") {
      toggle(path.search());
    }

    // To open design system => OS modifier key + d
    if (OSKey && event.code === "KeyD") {
      toggle(path.designSystemGuide());
    }

    // To create a new project => OS modifier key + n
    if (OSKey && event.code === "KeyN") {
      toggle(path.createProject());
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
