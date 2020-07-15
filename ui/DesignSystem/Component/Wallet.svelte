<script>
  import { Title } from "../Primitive";
  import Rad from "./Rad.svelte";
  import TransactionList from "./Wallet/TxList.svelte";
  import SendReceive from "./Wallet/SendReceive.svelte";
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
</style>

<div class="container">
  <div>
    <div class="balance">
      <Title style="padding-bottom: 1rem;" variant="large">Balance</Title>
      <Rad size="big" rad="234" usd="74" />
    </div>
    <div class="send-receive">
      <SendReceive />
    </div>
  </div>
  <div class="transactions">
    <Title variant="large" style="padding: 1.25rem 1.5rem;">Transactions</Title>
    <TransactionList />
  </div>
</div>
