<script lang="typescript">
  import Button from "ui/DesignSystem/Button.svelte";
  import Icon from "ui/DesignSystem/Icon";
  import TextInput from "ui/DesignSystem/TextInput.svelte";

  import Receiver from "./Receiver.svelte";

  import {
    ReceiverStatus,
    receiverStore,
    receiverValidationStore,
  } from "ui/src/funding/pool";
  import type * as pool from "ui/src/funding/pool";
  import { ValidationStatus } from "ui/src/validation";

  // The current list of receivers
  export let receivers: pool.Receivers = new Map();
  export let updating = false;
  export let editing = false;
  export let style = "";
  export let alignment = "left"; // "center";

  function toggleReceiver(address: pool.Address) {
    const status = receivers.get(address);
    if (!status) {
      return;
    }

    switch (status) {
      case ReceiverStatus.Added:
        receivers.delete(address);
        break;
      case ReceiverStatus.Present:
        receivers.set(address, ReceiverStatus.Removed);
        break;
      case ReceiverStatus.Removed:
        if (receivers.has(address)) {
          receivers.set(address, ReceiverStatus.Present);
        } else {
          receivers.delete(address);
        }
        break;
    }
    refresh();
  }

  function addNew(address: pool.Address) {
    if (receivers.has(address)) {
      newValue = "";
      return;
    }

    receivers.set(address, ReceiverStatus.Added);
    receivers = receivers;
    newValue = "";
    refresh();
  }

  function refresh() {
    receivers = receivers;
  }

  // The input field value
  let newValue: string = "";

  $: sortedEntries = [...receivers].sort(([_a, statusA], [_b, statusB]) => {
    const s = [
      ReceiverStatus.Removed,
      ReceiverStatus.Present,
      ReceiverStatus.Added,
    ];
    return s.indexOf(statusA) > s.indexOf(statusB) ? -1 : 1;
  });

  let validating = false;
  $: validation = receiverValidationStore();
  $: receiverStore.set(newValue);
  $: {
    if ($receiverStore && $receiverStore.length > 0) {
      validating = true;
    }
    if (validating) {
      validation.validate($receiverStore);
    }
  }

  let disabled = true;
  $: disabled = $validation.status !== ValidationStatus.Success;
</script>

<style>
  .row {
    display: flex;
    align-items: center;
  }

  .receivers.centered .row {
    justify-content: center;
  }

  .list {
    flex-wrap: wrap;
    gap: 0.625rem;
  }

  .row + .row {
    margin-top: 0.625rem;
  }
</style>

<div class="receivers" {style} class:centered={alignment === "center"}>
  <div class="row list">
    {#each sortedEntries as [address, status]}
      <Receiver
        onClick={editing ? x => toggleReceiver(x) : undefined}
        {address}
        {status}
        disabled={updating} />
    {/each}
  </div>

  {#if editing}
    <div class="row">
      <TextInput
        disabled={updating}
        bind:value={newValue}
        placeholder="Enter an Ethereum address"
        style="min-width: 360px" />

      <Button
        disabled={disabled || updating}
        on:click={() => addNew(newValue)}
        variant="outline"
        style="margin-left: 8px; border-color: var(--color-foreground-level-3)">
        <Icon.Plus style="display: flex" />
      </Button>
    </div>
  {/if}
</div>
