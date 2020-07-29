<script>
  import { nFormatter } from "../../src/transaction.ts";
  import { Title } from "../Primitive";
  import Rad from "./Rad.svelte";
  import TransactionList from "./Wallet/TxList.svelte";
  import SendReceive from "./Wallet/SendReceive.svelte";
  import Receive from "./Wallet/Receive.svelte";

  export let dataCy = null;
  export let transactions = null;
  export let balance = null;
  export let accountId = null;
  export let id = null;
</script>

<style>
  .container {
    display: flex;
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  .balance,
  .send-receive,
  .transactions {
    border-radius: 4px;
    border: 1px solid var(--color-foreground-level-2);
  }

  .transactions {
    width: 100%;
    height: 100%;
    margin-left: 1.5rem;
  }

  .balance {
    width: 20rem;
    padding: 1.25rem 1.5rem;
    margin-bottom: 1.5rem;
  }
  .send-receive {
    width: 20rem;
  }
  .empty-state {
    width: 20rem;
    margin: 0 auto;
    padding-top: 6rem;
  }
</style>

<div class="container" data-cy={dataCy}>
  {#if balance !== '0' || transactions.length !== 0}
    <div>
      <div class="balance" data-cy="balance">
        <Title style="padding-bottom: 1rem;" variant="large">Balance</Title>
        <Rad
          style="display: inline-block;"
          size="big"
          rad={nFormatter(balance)}
          usd={nFormatter(balance)} />
      </div>
      <div class="send-receive" data-cy="send-receive">
        <SendReceive {accountId} {id} />
      </div>
    </div>
    <div class="transactions" data-cy="transactions">
      <Title
        variant="large"
        style="padding: 1.25rem 1.5rem; border-bottom: 1px solid
        var(--color-foreground-level-2);">
        Transactions
      </Title>
      <TransactionList {transactions} />
    </div>
  {:else}
    <div class="empty-state">
      <Receive
        {accountId}
        text="To get started, buy some RADs and transfer them here." />
    </div>
  {/if}
</div>
