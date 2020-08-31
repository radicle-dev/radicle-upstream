<script>
  import * as notification from "../../src/notification.ts";
  import * as remote from "../../src/remote.ts";

  import WithContext from "./WithContext.svelte";

  export let store = null;
  export let context = null;

  // TODO(sos): svelte-check doesn't like the $$props getter; figure out
  // ergonomic alternative
  const errorSlot = $$props.$$slots.error;
  $: if ($store.status === remote.Status.Error && !errorSlot) {
    console.error("Remote error", $store.error);
    notification.error($store.error.message);
  }
</script>

{#if $store.status === remote.Status.NotAsked}
  <slot name="not-asked" />
{:else if $store.status === remote.Status.Loading}
  <slot name="loading" />
{:else if $store.status === remote.Status.Success}
  {#if context && typeof context === 'string'}
    <WithContext data={$store.data} name={context}>
      <slot data={$store.data} />
    </WithContext>
  {:else}
    <slot data={$store.data} />
  {/if}
{:else if $store.status === remote.Status.Error}
  <slot name="error" error={$store.error} />
{/if}
