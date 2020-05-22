<script>
  import { location, pop, push } from "svelte-spa-router";

  import * as path from "../src/path.ts";
  import * as transaction from "../src/transaction.ts";

  const toggle = (destination) => {
    if (path.active(destination, $location)) {
      pop();
    }
    push(destination);
  };

  const onKeydown = (event) => {
    if (event.target !== document.body) {
      return false;
    }

    if (event.shiftKey && event.code === "KeyD") {
      toggle(path.designSystemGuide());
    }

    // TODO(xla): Remove once we have tx polling.
    if (event.shiftKey && event.code === "KeyT") {
      transaction.fetchList();
    }

    if (event.shiftKey && event.code === "Slash") {
      toggle(path.help());
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />
