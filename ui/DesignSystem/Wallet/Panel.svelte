<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as ethers from "ethers";

  import Icon from "ui/DesignSystem/Icon";
  import Identifier from "ui/DesignSystem/Identifier.svelte";
  import Tooltip from "ui/DesignSystem/Tooltip.svelte";
  import Button from "ui/DesignSystem/Button.svelte";

  import { config } from "ui/src/config";
  import * as ethereum from "ui/src/ethereum";

  export let dai: ethers.BigNumber | null;
  export let eth: ethers.BigNumber | null;
  export let rad: ethers.BigNumber | null;
  export let address: string;
  export let onDisconnect: () => void;
  export let style = "";

  function formatBalance(balance: ethers.BigNumber | null): string {
    if (balance === null) {
      return "–";
    } else {
      return ethereum.formatTokenAmount(balance);
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
    <h2 data-cy="eth-balance">
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
    <Identifier
      name="wallet address"
      value={address}
      kind="ethAddress"
      showIcon={false} />
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
