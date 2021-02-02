<script lang="typescript">
  import { blur } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { store } from "../../src/notification";

  import Notification from "./Notification.svelte";

  export let style = "";
</script>

<style>
  .notification-faucet {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;

    z-index: 1001;
  }
</style>

<div class="notification-faucet" {style}>
  {#each $store as notification (notification.id)}
    <div
      data-cy="notification"
      style="max-width: 95%;"
      animate:flip
      transition:blur={{ duration: 300 }}>
      <Notification {notification} />
    </div>
  {/each}
</div>
