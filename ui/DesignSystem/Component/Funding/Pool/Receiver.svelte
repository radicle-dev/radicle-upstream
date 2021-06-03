<script lang="typescript">
  import { Copyable } from "../../../Component";
  import { Button, Icon } from "../../../Primitive";
  import type { Address, ReceiverStatus } from "../../../../src/funding/pool";
  import { ellipsed } from "../../../../src/style";

  export let address: Address = "";
  export let disabled = false;
  export let status: ReceiverStatus;
  export let onClick: ((title: string) => void) | undefined;
</script>

<style>
  .receiver {
    display: flex;
    align-items: center;
    justify-content: space-around;

    height: 2.75rem;
    padding: 0px 1rem;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
  }

  .receiver.removed {
    opacity: 0.35;
  }

  .content {
    padding-left: 0.25rem;
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
      on:click={() => (onClick ? onClick(address) : {})}
      {disabled}
      variant="embedded"
      icon={Icon.Cross}
      style="padding: 0" />
  {/if}
  <p class="content typo-text-bold">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={address}
      notificationText="Address copied to the clipboard"
      style="padding-left: 0rem">
      {ellipsed(address, 4)}
    </Copyable>
  </p>
</span>
