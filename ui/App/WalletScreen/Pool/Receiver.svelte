<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Address, ReceiverStatus } from "ui/src/funding/pool";

  import Button from "ui/DesignSystem/Button.svelte";
  import Icon from "ui/DesignSystem/Icon";
  import Identifier from "ui/DesignSystem/Identifier.svelte";

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
    padding: 0 1rem;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
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
      on:click={() => (onClick ? onClick(address) : {})}
      {disabled}
      variant="embedded"
      icon={Icon.Cross}
      style="padding: 0; margin-right: 0.5rem;" />
  {/if}
  <p class="content">
    <Identifier value={address} kind="ethAddress" showIcon={false} />
  </p>
</span>
