<script lang="typescript">
  import { blur } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { store } from "../../src/notification";

  import Notification from "./Notification.svelte";

  export let style = "";
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    position: fixed;
    bottom: 2rem;
    z-index: 1001;
    left: var(--sidebar-width);
    width: calc(100vw - var(--sidebar-width));
  }
</style>

<div class="wrapper" {style}>
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
