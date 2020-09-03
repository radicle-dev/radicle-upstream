<script>
  import { copyToClipboard } from "../../../native/ipc.js";
  import * as notification from "../../src/notification.ts";

  export let style = null;
  export let value = null;
  export let notificationText = "Copied to your clipboard";

  let copied = false;

  const copy = () => {
    if (copied) {
      return;
    }

    copyToClipboard(value);

    notification.info(notificationText);

    copied = true;

    setTimeout(() => {
      copied = false;
    }, 1000);
  };
</script>

<style>
  .wrapper {
    cursor: pointer;
    display: inline-flex;
    white-space: nowrap;
  }
</style>

<div class="wrapper" on:click|stopPropagation={copy}>
  <span class="content" {style}>
    <slot />
  </span>
</div>
