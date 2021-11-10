<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Readable } from "svelte/store";

  import * as notification from "ui/src/notification";
  import * as remote from "ui/src/remote";

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let store: Readable<remote.Data<any>>;

  export let disableErrorLogging: boolean = false;

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
    notification.showException(storeValue.error);
  }

  $: data =
    $store.status === remote.Status.Success
      ? ($store as remote.SuccessState).data
      : undefined;
</script>

{#if $store.status === remote.Status.NotAsked}
  <slot name="not-asked" />
{:else if $store.status === remote.Status.Loading}
  <slot name="loading" />
{:else if $store.status === remote.Status.Success}
  {#if (Array.isArray(data) && data.length === 0) || data === null}
    <slot name="empty" />
  {:else}
    <slot {data} />
  {/if}
{:else if storeValue.status === remote.Status.Error}
  <slot name="error" error={storeValue.error}>
    <div bind:this={noErrorSlotProvided} />
  </slot>
{/if}
