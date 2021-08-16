<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { BigNumber } from "ethers";

  import Copyable from "ui/DesignSystem/Copyable.svelte";
  import Icon from "ui/DesignSystem/Icon";
  import Tooltip from "ui/DesignSystem/Tooltip.svelte";

  import { config } from "ui/src/config";
  import * as ethereum from "ui/src/ethereum";
  import { ellipsed } from "ui/src/style";
  import { Button } from "ui/DesignSystem";

  export let dai: BigNumber | null;
  export let eth: BigNumber | null;
  export let rad: BigNumber | null;
  export let address: string;
  export let onDisconnect: () => void;
  export let style = "";

  function formatBalance(balance: BigNumber | null): string {
    if (balance === null) {
      return "–";
    } else {
      return ethereum.toBaseUnit(balance).toNumber().toLocaleString("en-US");
    }
  }
</script>

<style>
  .panel {
    display: flex;
    flex-direction: column;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;

    padding-bottom: 0px;
  }

  .balances {
    padding: 1.5rem;
  }

  h2 {
    margin: 0.75rem 0 1.5rem;
    color: var(--color-primary);
  }

  .supported {
    color: var(--color-foreground-level-5);
  }

  .supported h3 {
    padding-top: 0.375rem;
  }

  .address-box {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;

    border-top: 1px solid var(--color-foreground-level-2);
    color: var(--color-foreground-level-6);
  }
</style>

<div class="panel" {style}>
  <div class="balances">
    <h3>Balance</h3>
    <h2>
      {formatBalance(eth)} ETH
    </h2>
    <div class="supported">
      <h5>Supported tokens</h5>
      <h3>
        {formatBalance(rad)} RAD
      </h3>
      {#if config.isDev}
        <h3>
          {formatBalance(dai)} DAI
        </h3>
      {/if}
    </div>
  </div>

  <div class="address-box typo-text">
    <Copyable
      showIcon={false}
      styleContent={false}
      style="padding-left: 0;"
      copyContent={address}
      notificationText="Address copied to the clipboard">
      {ellipsed(address)}
    </Copyable>
    <Tooltip value="Disconnect">
      <Button
        style="padding:0.5rem;"
        on:click={onDisconnect}
        variant="transparent">
        <Icon.Cross />
      </Button>
    </Tooltip>
  </div>
</div>
