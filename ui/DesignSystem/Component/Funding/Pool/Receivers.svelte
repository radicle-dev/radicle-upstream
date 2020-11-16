<script lang="typescript">
  import { Button, Icon, Input } from "../../../Primitive";

  import Receiver from "./Receiver.svelte";

  import { AddressStatus } from "../../../../src/funding/pool";
  import type { Changeset, Address } from "../../../../src/funding/pool";

  // The current list of receivers
  export let receivers: Address[] = [];
  export let updating = false;
  export let editing = false;

  // Read-only for foreginers
  export let changeset: Changeset = new Map();

  $: updating, refreshChangeset();

  function refreshChangeset(): Changeset {
    // Refresh the changeset only when something changed
    // **after** updating, not during. By doing so we keep
    // displaying the changes that are being awaiting inclusion.
    if (!updating) {
      changeset = new Map(receivers.map(r => [r, AddressStatus.Present]));
    }
  }

  function toggleReceiver(x: Address) {
    const status = changeset.get(x);

    switch (status) {
      case AddressStatus.Added:
        changeset.delete(x);
        break;
      case AddressStatus.Present:
        receivers = receivers.filter(r => r !== x);
        changeset.set(x, AddressStatus.Removed);
        break;
      case AddressStatus.Removed:
        if (receivers.includes(x)) {
          changeset.set(x, AddressStatus.Present);
        } else {
          changeset.delete(x);
        }
        break;
    }
    refresh();
  }

  function addNew(x: Address) {
    if (changeset.has(x)) return;
    changeset.set(x, AddressStatus.Added);
    receivers.push(x);
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
    {#each [...changeset.entries()] as [address, status]}
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
        disabled={updating}
        on:click={() => addNew(newValue)}
        variant="outline"
        style="margin-left: 8px; border-color: var(--color-foreground-level-3)">
        <Icon.Plus />
      </Button>
    {/if}
  </div>
</div>
