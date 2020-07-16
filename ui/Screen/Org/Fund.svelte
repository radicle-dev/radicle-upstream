<script>
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  export let params = null;

  const orgTransactions = transactions => {
    return transactions.filter((tx) => {
      return "domainId" in tx.messages[0]
        ? tx.messages[0].domainId == params.id
        : null;
    });
  };
</script>

<Remote {store} let:data={transactions}>
  <Wallet transactions={orgTransactions(transactions)} />
</Remote>
