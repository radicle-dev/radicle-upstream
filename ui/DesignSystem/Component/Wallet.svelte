<script>
  import { Title } from "../Primitive";
  import Rad from "./Rad.svelte";
  import WalletTransactionList from "./Wallet/TransactionList.svelte";
  import WalletSendReceive from "./Wallet/SendReceive.svelte";
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    display: grid;
    grid-template-columns: 20rem auto;
    grid-gap: 1.5rem;
  }

  .balance,
  .send-receive,
  .transactions {
    border-radius: 4px;
    border: 1px solid var(--color-foreground-level-2);
    margin-bottom: 1.5rem;
  }

  .balance {
    padding: 1.25rem 1.5rem;
    display: flex;
    justify-content: space-between;
  }

  .send-receive {
    height: 25.5rem;
  }
</style>

<div class="container">
  <div>
    <div class="balance">
      <Title variant="large">Balance</Title>
      <Rad size="big" rad="234" usd="74" />
    </div>
    <div class="send-receive">
      <WalletSendReceive />
    </div>
  </div>
  <div class="transactions">
    <Title variant="large" style="padding: 1.25rem 1.5rem;">Transactions</Title>
    <WalletTransactionList />
  </div>

</div>
