<script lang="typescript">
  import type { SvelteComponent } from "svelte";

  import { Copyable } from "../../../Component";
  import { Button, Icon } from "../../../Primitive";
  import { displayAddress, AddressStatus } from "../../../../src/funding/pool";
  import type { Address } from "../../../../src/funding/pool";

  export let address: Address = "";
  export let disabled = false;
  export let status: AddressStatus;
  export let onClick: (title: string) => void;

  function iconForStatus(s: AddressStatus): SvelteComponent {
    switch (s) {
      case AddressStatus.Added:
      case AddressStatus.Present:
        return Icon.Cross;
      case AddressStatus.Removed:
        return Icon.ChevronLeft;
    }
  }
</script>

<style>
  .receiver {
    display: flex;
    align-items: center;
    justify-content: space-around;

    padding: 0px 20px 0px 10px;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    margin-right: 12px;
  }

  .receiver:after {
    top: -10px;
    right: -10px;
    position: relative;
  }

  .receiver.removed {
    border-color: var(--color-negative);
    text-decoration: line-through;
  }

  .receiver.removed,
  .receiver.removed .content {
    color: var(--color-negative);
  }

  .receiver.added {
    border-color: var(--color-positive);
    color: var(--color-positive);
  }
  .receiver.added,
  .receiver.added .content {
    color: var(--color-positive);
  }

  .content {
    color: var(--color-foreground-level-6);
  }

  .receiver:hover p,
  .receiver.disabled .content {
    color: var(--color-foreground-level-3) !important;
  }
</style>

<span class="receiver {status.toLowerCase()}" class:disabled>
  <Button
    on:click={() => onClick(address)}
    {disabled}
    variant="embedded"
    icon={iconForStatus(status)} />
  <p class="content typo-text-bold">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={address}
      notificationText="Address copied to the clipboard">
      {displayAddress(address)}
    </Copyable>
  </p>
</span>
