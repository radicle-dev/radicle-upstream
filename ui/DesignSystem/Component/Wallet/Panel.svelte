<script lang="typescript">
  import { Button, Icon } from "../../Primitive";
  import { Dai, Copyable, Overlay } from "../../Component";

  import * as wallet from "../../../src/wallet";
  import { ellipsed } from "../../../src/style";

  export let account: wallet.Account;
  export let onDisconnect: () => void;
  export let style = "";

  $: balanceFontSize = `min(36px, calc(20rem / ${Math.max(
    account.balance.toString().length,
    1
  )}))`;

  let expanded = false;
</script>

<style>
  .panel {
    display: flex;
    flex-direction: column;

    width: 25rem;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;

    padding-top: var(--content-padding);
    padding-bottom: 0px;
  }

  h3,
  h1 {
    padding: 0 2rem;
  }

  h1 {
    margin: 1.35rem 0 1rem 0;
    color: var(--color-secondary);
  }

  .menu {
    position: relative;
    right: 9rem;
    width: fit-content;
    padding-right: 1rem;

    box-shadow: var(--elevation-medium);
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.25rem;
    user-select: none;
    background-color: var(--color-background);
    overflow: hidden; /* hack to make inner option corners rounded */
    z-index: 1;
  }

  .address-box {
    display: flex;
    align-items: center;
    justify-content: space-between;

    border-top: 1px solid var(--color-foreground-level-2);
    padding: 1rem 2rem;
    color: var(--color-foreground-level-6);
  }
</style>

<div class="panel" {style}>
  <h3>Balance</h3>
  <h1>
    <Dai
      color="var(--color-secondary)"
      size="h1"
      style={`font-size: ${balanceFontSize}`}>
      {wallet.formattedBalance(account.balance.toNumber())}
    </Dai>
  </h1>

  <div class="address-box typo-text">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={account.address}
      notificationText="Address copied to the clipboard">
      {ellipsed(account.address)}
    </Copyable>

    <Overlay
      {expanded}
      on:hide={() => (expanded = false)}
      style="overflow: visible; width: 32px; height: 40px;">
      <Button
        variant="embedded"
        on:click={() => (expanded = !expanded)}
        style="color: var(--color-foreground-level-3);">
        <Icon.ChevronDown />
      </Button>
      <div class="menu" hidden={!expanded}>
        <Button icon={Icon.Cross} variant="embedded" on:click={onDisconnect}>
          <p class="typo-text">Disconnect</p>
        </Button>
      </div>
    </Overlay>
  </div>
</div>
