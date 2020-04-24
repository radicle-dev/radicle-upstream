<script>
  import * as remote from "../../src/remote.ts";

  export let store = null;
</script>

{#if $store.status === remote.Status.NotAsked}
  <slot name="not-asked" />
{:else if $store.status === remote.Status.Loading}
  <slot name="loading" />
{:else if $store.status === remote.Status.Success}
  <slot data={$store.data} />
{:else if $store.status === remote.Status.Error}
  <slot name="error" error={$store.error} />
{/if}
