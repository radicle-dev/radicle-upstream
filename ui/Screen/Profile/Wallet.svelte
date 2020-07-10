<script>
  import { Title } from "../../DesignSystem/Primitive";
  import {
    Rad,
    WalletTransactionList,
    WalletSendReceive,
  } from "../../DesignSystem/Component";
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    display: grid;
    grid-template-columns: auto 20rem;
    grid-template-rows: auto;
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
</style>

<div class="container">
  <div>
    <div class="balance">
      <Title variant="large">Balance</Title>
      <Rad size="big" rad="234" usd="74" />
    </div>
    <div class="transactions">
      <Title variant="large" style="padding: 1.25rem 1.5rem;">
        Transactions
      </Title>
      <WalletTransactionList />
    </div>
  </div>
  <div class="send-receive">
    <WalletSendReceive />
  </div>

</div>
