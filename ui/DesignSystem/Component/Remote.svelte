<script lang="ts">
  import type { Readable } from "svelte/store";

  import * as notification from "../../src/notification";
  import * as error from "../../src/error";
  import * as remote from "../../src/remote";

  import WithContext from "./WithContext.svelte";

  export let store: Readable<remote.Data<any>>;
  export let context: string | undefined = undefined;

  // Shorthand for casting these states
  type ErrorState = { status: remote.Status.Error; error: error.Error };
  type SuccessState = { status: remote.Status.Success; data: any };

  // If no error slot was provided, svelte will instantiate the fallback div
  let noErrorSlotProvided: HTMLDivElement;

  $: if ($store.status === remote.Status.Error && !!noErrorSlotProvided) {
    console.error("Remote error", ($store as ErrorState).error);
    notification.error(($store as ErrorState).error.message);
  }

  $: data =
    $store.status === remote.Status.Success
      ? ($store as SuccessState).data
      : undefined;

  $: remoteError =
    $store.status === remote.Status.Error
      ? ($store as ErrorState).error
      : undefined;
</script>

{#if $store.status === remote.Status.NotAsked}
  <slot name="not-asked" />
{:else if $store.status === remote.Status.Loading}
  <slot name="loading" />
{:else if $store.status === remote.Status.Success}
  {#if context}
    <WithContext {data} name={context}>
      <slot {data} />
    </WithContext>
  {:else}
    <slot {data} />
  {/if}
{:else if $store.status === remote.Status.Error}
  <slot name="error" error={remoteError}>
    <div bind:this={noErrorSlotProvided} />
  </slot>
{/if}
