<script>
  import { blur } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { store } from "../../src/notification.ts";

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
    <div
      data-cy="notification"
      animate:flip
      transition:blur={{ duration: 300 }}>
      <Notification
        showIcon={notification.showIcon}
        level={notification.level}
        message={notification.message}
        actionText={notification.actionText}
        on:action={notification.actionHandler} />
    </div>
  {/each}
</div>
