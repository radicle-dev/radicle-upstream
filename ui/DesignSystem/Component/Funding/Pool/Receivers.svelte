<script lang="typescript">
  import { Button, Icon, Input } from "../../../Primitive";

  import Receiver from "./Receiver.svelte";

  import { ReceiverStatus } from "../../../../src/funding/pool";
  import * as pool from "../../../../src/funding/pool";

  // The current list of receivers
  export let receivers: pool.Receivers = new Map();
  export let updating = false;
  export let editing = false;
  export let style = "";

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
    if (receivers.has(address)) return;
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
</script>

<style>
  .receivers {
    padding: var(--content-padding);
  }
  .row {
    display: flex;
    align-items: center;
  }

  .list {
    flex-wrap: wrap;
  }

  .row + .row {
    margin-top: 10px;
  }
</style>

<div class="receivers" {style}>
  <div class="row list">
    {#each sortedEntries as [address, status]}
      <Receiver
        onClick={editing ? x => toggleReceiver(x) : undefined}
        {address}
        {status}
        disabled={updating} />
    {/each}
  </div>

  <div class="row">
    {#if editing}
      <Input.Text
        disabled={updating}
        bind:value={newValue}
        placeholder="Enter an Ethereum address or a Radicle handle"
        style="min-width: 380px" />

      <Button
        disabled={newValue.trim().length === 0 || updating}
        on:click={() => addNew(newValue)}
        variant="outline"
        style="margin-left: 8px; border-color: var(--color-foreground-level-3)">
        <Icon.Plus />
      </Button>
    {/if}
  </div>
</div>
