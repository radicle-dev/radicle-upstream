<script>
  import { blur } from "svelte/transition";

  import { remove, store } from "../../src/notification.ts";

  import Notification from "./Notification.svelte";

  export let style = null;
</script>

<style>
  .wrapper {
    position: absolute;
    bottom: 32px;
    z-index: 1001;
    left: 50%;
    transform: translateX(-50%);
  }

  .notification {
    margin-bottom: 8px;
  }
</style>

<div class="wrapper" {style}>
  {#each $store as notification (notification.id)}
    <div class="notification" transition:blur={{ duration: 300 }}>
      <Notification
        level={notification.level}
        message={notification.message}
        on:click={() => remove(notification.id)} />
    </div>
  {/each}
</div>
