<script>
  import * as remote from "../../src/remote.ts";

  export let store = null;
</script>

{#if $store.status === remote.Status.NotAsked}
  <slot name="not-asked"></slot>
{:else if $store.status === remote.Status.Loading}
  <slot name="loading"></slot>
{:else if $store.status === remote.Status.Success}
  <slot name="success" data={$store.data}></slot>
{:else if $store.status === remote.Status.Error}
  <slot name="error" error={$store.error}></slot>
{/if}
