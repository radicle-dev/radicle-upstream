<script lang="typescript">
  import { Copyable } from "../../../Component";
  import { Button, Icon } from "../../../Primitive";
  import { displayAddress, ReceiverStatus } from "../../../../src/funding/pool";
  import type { Address } from "../../../../src/funding/pool";

  export let address: Address = "";
  export let disabled = false;
  export let status: ReceiverStatus;
  export let onClick: (title: string) => void | undefined;
</script>

<style>
  .receiver {
    display: flex;
    align-items: center;
    justify-content: space-around;

    height: 42px;
    padding: 0px 20px 0px 10px;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
  }

  .receiver.removed {
    opacity: 0.35;
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
  {#if onClick}
    <Button
      on:click={() => onClick(address)}
      {disabled}
      variant="embedded"
      icon={Icon.Cross} />
  {/if}
  <p class="content typo-text-bold">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={address}
      notificationText="Address copied to the clipboard">
      {displayAddress(address, 4)}
    </Copyable>
  </p>
</span>
