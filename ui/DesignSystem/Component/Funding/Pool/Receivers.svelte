<script lang="typescript">
  import { Button, Icon, Input } from "../../../Primitive";

  import Receiver from "./Receiver.svelte";

  import {
    ReceiverStatus,
    receiverStore,
    receiverValidationStore,
  } from "../../../../src/funding/pool";
  import * as pool from "../../../../src/funding/pool";
  import { ValidationStatus } from "../../../../src/validation";

  // The current list of receivers
  export let receivers: pool.Receivers = new Map();
  export let updating = false;
  export let editing = false;
  export let style = "";
  export let alignment = "left"; // "center";

  function toggleReceiver(x: pool.Address) {
    const status = receivers.get(x);
    console.log("toggleReceiver", x);
    if (!status) return;

    switch (status) {
      case ReceiverStatus.Added:
        receivers.delete(x);
        break;
      case ReceiverStatus.Present:
        receivers.set(x, ReceiverStatus.Removed);
        break;
      case ReceiverStatus.Removed:
        if (receivers.has(x)) {
          receivers.set(x, ReceiverStatus.Present);
        } else {
          receivers.delete(x);
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
    if ($receiverStore && $receiverStore.length > 0) validating = true;
    if (validating) validation.validate($receiverStore);
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
    gap: 10px;
  }

  .row + .row {
    margin-top: 10px;
  }
</style>

<div class="receivers" {style} class:centered={alignment === 'center'}>
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
      <Input.Text
        disabled={updating}
        bind:value={newValue}
        placeholder="Enter an Ethereum address"
        style="min-width: 360px" />

      <Button
        disabled={disabled || updating}
        on:click={() => addNew(newValue)}
        variant="outline"
        style="margin-left: 8px; border-color: var(--color-foreground-level-3)">
        <Icon.Plus />
      </Button>
    </div>
  {/if}
</div>
