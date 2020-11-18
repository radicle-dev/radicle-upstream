<script lang="typescript">
  import type { Readable } from "svelte/store";

  import * as error from "../../src/error";
  import * as remote from "../../src/remote";

  import WithContext from "./WithContext.svelte";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let store: Readable<remote.Data<any>>;
  export let context: string | undefined = undefined;

  export let disableErrorLogging: boolean = false;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type SuccessState = { status: remote.Status.Success; data: any };

  // If no error slot was provided, svelte will instantiate the fallback div
  let noErrorSlotProvided: HTMLDivElement;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let storeValue: remote.Data<any>;
  $: storeValue = $store;

  $: if (
    storeValue.status === remote.Status.Error &&
    !!noErrorSlotProvided &&
    !disableErrorLogging
  ) {
    error.show({
      code: error.Code.RemoteStoreError,
      message: storeValue.error.message,
      source: storeValue.error,
    });
  }

  $: data =
    $store.status === remote.Status.Success
      ? ($store as SuccessState).data
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
{:else if storeValue.status === remote.Status.Error}
  <slot name="error" error={storeValue.error}>
    <div bind:this={noErrorSlotProvided} />
  </slot>
{/if}
