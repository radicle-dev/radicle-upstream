<script lang="typescript">
  import type { Readable } from "svelte/store";

  import * as error from "../../src/error";
  import * as notification from "../../src/notification";
  import * as remote from "../../src/remote";

  import WithContext from "./WithContext.svelte";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let store: Readable<remote.Data<any>>;
  export let context: string | undefined = undefined;

  // If no error slot was provided, svelte will instantiate the fallback div
  let noErrorSlotProvided: HTMLDivElement;

  $: if ($store.status === remote.Status.Error && !!noErrorSlotProvided) {
    console.error("Remote error", ($store as remote.ErrorState).error);
    notification.error(($store as remote.ErrorState).error.message);
  }

  $: data =
    $store.status === remote.Status.Success
      ? ($store as remote.SuccessState).data
      : undefined;

  $: remoteError =
    $store.status === remote.Status.Error
      ? ($store as remote.ErrorState).error
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
  {:else if (Array.isArray(data) && data.length === 0) || data === null}
    <slot name="empty" />
  {:else}
    <slot {data} />
  {/if}
{:else if $store.status === remote.Status.Error}
  <slot name="error" error={remoteError}>
    <div bind:this={noErrorSlotProvided} />
  </slot>
{/if}
