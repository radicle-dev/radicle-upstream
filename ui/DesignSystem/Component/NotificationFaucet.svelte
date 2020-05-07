<script>
  import { onDestroy } from "svelte";
  import { blur } from "svelte/transition";

  import { NOTIFICATION_TIMEOUT } from "../../config.js";
  import { store } from "../../src/notification.ts";

  import Notification from "./Notification.svelte";

  export let style = null;
  let messages = [];

  const unsubscribe = store.subscribe((message) => {
    if (!message) {
      return;
    }

    const id = Math.random();
    messages = [
      ...messages,
      { id: id, message: message.text, level: message.level },
    ];
    store.set(null);
    setTimeout(() => {
      remove(id);
    }, NOTIFICATION_TIMEOUT);
  });

  const remove = (id) => {
    messages = messages.filter((m) => m.id != id);
  };

  onDestroy(() => {
    unsubscribe();
  });
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
  {#each messages as message}
    <div class="notification" transition:blur={{ duration: 300 }}>
      <Notification
        level={message.level}
        message={message.message}
        on:click={() => {
          remove(message.id);
        }} />
    </div>
  {/each}
</div>
