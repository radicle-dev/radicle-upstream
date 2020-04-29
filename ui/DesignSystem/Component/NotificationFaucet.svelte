<script>
  import { onDestroy } from "svelte";

  import { notificationStore } from "../../store/notification.js";
  import { NOTIFICATION_TIMEOUT } from "../../config.js";

  import Notification from "./Notification.svelte";

  import { blur } from "svelte/transition";

  export let style = null;
  let messages = [];

  const unsubscribe = notificationStore.subscribe(message => {
    if (!message) {
      return;
    }

    const id = Math.random();
    messages = [
      ...messages,
      { id: id, message: message.text, level: message.level }
    ];
    notificationStore.set();
    setTimeout(() => {
      remove(id);
    }, NOTIFICATION_TIMEOUT);
  });

  const remove = id => {
    messages = messages.filter(m => m.id != id);
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
        variant={message.level}
        on:click={() => {
          remove(message.id);
        }}>
        {message.message}
      </Notification>
    </div>
  {/each}
</div>
