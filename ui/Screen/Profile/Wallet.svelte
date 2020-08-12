<script>
  import { getContext } from "svelte";
  import { updateBalance, balance as balanceStore } from "../../src/account.ts";
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  const userTransactions = transactions => {
    return transactions.filter(tx => {
      return tx.messages[0].domainType !== "org";
    });
  };

  const session = getContext("session");
  $: accountId = session.identity ? session.identity.accountId : null;
  $: id = session.identity ? session.identity.metadata.handle : null;
  $: updateBalance(accountId);
</script>

<Remote {store} let:data={transactions}>
  <Remote store={balanceStore} let:data={balance}>
    <Wallet
      dataCy="user-wallet"
      transactions={userTransactions(transactions)}
      {balance}
      {accountId}
      {id} />
  </Remote>
</Remote>
