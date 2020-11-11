<script lang="typescript">
  import { Button } from "../../Primitive";
  import { Dai, Copyable } from "../../Component";

  import * as wallet from "../../../src/wallet";
  import { displayAddress } from "../../../src/funding/pool";

  export let account: wallet.Account;
  export let onDisconnect: () => void;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;

    width: 35vw;
    max-width: 400px;

    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;

    padding-top: var(--content-padding);
    padding-bottom: 0px;
  }

  h3,
  h1 {
    padding: 0 var(--content-padding);
  }

  h1 {
    margin: 1.35rem 0 1rem -3px;
    color: var(--color-secondary);
  }

  .address {
    display: flex;
    align-items: center;
    justify-content: space-between;

    border-top: 1px solid var(--color-foreground-level-2);
    padding: calc(var(--content-padding) / 2) var(--content-padding);
    color: var(--color-foreground-level-6);
  }
</style>

<div class="wrapper">
  <h3>Balance</h3>
  <h1>
    <Dai color={'var(--color-secondary)'} size={'h1'}>
      {wallet.abbreviateNumber(account.balance)}
    </Dai>
  </h1>

  <div class="address typo-text">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={account.address}
      notificationText="Address copied to the clipboard">
      {displayAddress(account.address)}
    </Copyable>

    <Button
      variant="transparent"
      on:click={onDisconnect}
      style="color: var(--color-foreground-level-3)">
      ↩
    </Button>
  </div>
</div>
