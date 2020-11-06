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
    justify-content: flex-end;
    align-items: center;

    text-align: center;
    padding: var(--content-padding) 0;
  }

  .balance,
  .address {
    color: var(--color-foreground-level-6);
  }

  .balance {
    display: flex;
    align-items: center;
  }

  .address {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    padding: 7px 14px;

    margin-left: 12px;
  }
</style>

<div class="wrapper">
  <p class="balance typo-text-bold">
    <Dai>{account.balance}</Dai>
  </p>
  <p class="address typo-text-bold">
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={account.address}
      notificationText="Address copied to the clipboard">
      {displayAddress(account.address)}
    </Copyable>
  </p>
  <Button
    style="margin-left: 7px;"
    variant="transparent"
    on:click={onDisconnect}>
    Disconnect
  </Button>
</div>
