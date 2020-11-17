<script lang="typescript">
  import { Button, Icon, Input } from "../../../Primitive";

  import Receiver from "./Receiver.svelte";

  import { AddressStatus } from "../../../../src/funding/pool";
  import * as pool from "../../../../src/funding/pool";

  // The current list of receivers
  export let receivers: pool.Receivers;
  export let updating = false;
  export let editing = false;

  let changeset: pool.Changeset = new Map();

  $: updating || editing, refreshChangeset();

  function refreshChangeset() {
    // Refresh the changeset only when something changed
    // **after** updating, not during. By doing so we keep
    // displaying the changes that are being awaiting inclusion.
    if (!updating) {
      changeset = new Map(
        [...receivers].map(([address, weight]) => [
          address,
          weight === 0 ? AddressStatus.Removed : AddressStatus.Present,
        ])
      );
    }
  }

  function toggleReceiver(x: pool.Address) {
    const status = changeset.get(x);

    switch (status) {
      case AddressStatus.Added:
        changeset.delete(x);
        receivers.delete(x);
        break;
      case AddressStatus.Present:
        receivers.set(x, 0);
        changeset.set(x, AddressStatus.Removed);
        break;
      case AddressStatus.Removed:
        if (receivers.has(x)) {
          receivers.set(x, 1);
          changeset.set(x, AddressStatus.Present);
        } else {
          receivers.delete(x);
          changeset.delete(x);
        }
        break;
    }
    refresh();
  }

  function addNew(address: pool.Address) {
    if (changeset.has(address)) return;
    changeset.set(address, AddressStatus.Added);
    receivers.set(address, 1);
    receivers = receivers;
    newValue = "";
    refresh();
  }

  function refresh() {
    receivers = receivers;
    changeset = changeset;
  }

  // The input field value
  let newValue: string = "";

  $: sortedEntries = [...changeset].sort(([_a, statusA], [_b, statusB]) => {
    const s = [AddressStatus.Removed, AddressStatus.Present, AddressStatus.Added];
    return s.indexOf(statusA) > s.indexOf(statusB) ? -1 : 1;
  });
</script>

<style>
  .receivers {
    margin: 1.5rem 0;
  }
  .row {
    display: flex;
    align-items: center;
  }

  .row + .row {
    margin-top: 10px;
  }
</style>

<div class="receivers">
  <div class="row">
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
