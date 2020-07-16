<script>
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  const userTransactions = transactions => {
    return transactions.filter((tx) => {
      return tx.messages[0].domainType !== "org";
    });
  };
</script>

<Remote {store} let:data={transactions}>
  <Wallet transactions={userTransactions(transactions)} />
</Remote>
