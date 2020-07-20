<script>
  import { transactions as txStore } from "../../src/transaction.ts";
  import { org, orgTransactions } from "../../src/org.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  export let params = null;
  const accountId = $org.data.accountId;
</script>

<Remote store={txStore} let:data={transactions}>
  <Wallet
    transactions={orgTransactions(transactions, params.id)}
    balance={3484}
    address={accountId} />
</Remote>
