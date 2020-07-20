<script>
  import { transactions as txStore } from "../../src/transaction.ts";
  import { org as orgStore } from "../../src/org.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  export let params = null;

  const orgTransactions = transactions => {
    return transactions.filter(tx => {
      return "domainId" in tx.messages[0]
        ? tx.messages[0].domainId == params.id
        : null;
    });
  };
</script>

<Remote store={txStore} let:data={transactions}>
  <Remote store={orgStore} let:data={org}>
    <Wallet
      transactions={orgTransactions(transactions)}
      balance={3484}
      address={org.accountId} />
  </Remote>
</Remote>
