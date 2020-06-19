<script>
  import { remove, store } from "../../src/notification.ts";

  import Notification from "./Notification.svelte";

  export let style = null;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: absolute;
    bottom: 24px;
    z-index: 1001;
    left: 50%;
    transform: translateX(-50%);
  }
</style>

<div class="wrapper" {style}>
  {#each $store as notification (notification.id)}
    <Notification
      dataCy="notification"
      level={notification.level}
      message={notification.message}
      on:click={() => remove(notification.id)} />
  {/each}
</div>
