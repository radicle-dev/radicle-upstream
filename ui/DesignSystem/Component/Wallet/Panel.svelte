<script lang="typescript">
  import { Icon } from "ui/DesignSystem/Primitive";
  import { Copyable, Tooltip } from "ui/DesignSystem/Component";

  import * as wallet from "ui/src/wallet";
  import { ellipsed } from "ui/src/style";

  export let account: wallet.Account;
  export let onDisconnect: () => void;
  export let style = "";
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
      {wallet.formattedBalance(account.ethBalance.toNumber())} ETH
    </h2>
    <div class="supported">
      <h5>Supported tokens</h5>
      <h3>
        {wallet.formattedBalance(account.daiBalance.toNumber())} DAI
      </h3>
    </div>
  </div>

  <div class="address-box typo-text">
    <Copyable
      showIcon={false}
      styleContent={false}
      style="padding-left: 0;"
      copyContent={account.address}
      notificationText="Address copied to the clipboard">
      {ellipsed(account.address)}
    </Copyable>
    <Tooltip value="Disconnect">
      <Icon.Cross
        on:click={onDisconnect}
        style="cursor: pointer; margin-top: 4px;" />
    </Tooltip>
  </div>
</div>
