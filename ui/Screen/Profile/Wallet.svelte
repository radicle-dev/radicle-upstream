<script>
  import { getContext } from "svelte";
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  const userTransactions = transactions => {
    return transactions.filter(tx => {
      return tx.messages[0].domainType !== "org";
    });
  };

  const session = getContext("session");
  $: accountId = session.identity ? session.identity.accountId : null;
</script>

<Remote {store} let:data={transactions}>
  <Wallet
    transactions={userTransactions(transactions)}
    balance={3552}
    {accountId} />
</Remote>
