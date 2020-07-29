<script>
  import { updateBalance, balance as balanceStore } from "../../src/account.ts";
  import { transactions as txStore } from "../../src/transaction.ts";
  import { org, orgTransactions } from "../../src/org.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  export let params = null;
  const accountId = $org.data.accountId;
  const id = $org.data.id;
  $: updateBalance(accountId);
</script>

<Remote store={txStore} let:data={transactions}>
  <Remote store={balanceStore} let:data={balance}>
    <Wallet
      dataCy="org-wallet"
      transactions={orgTransactions(transactions, params.id)}
      {balance}
      {accountId}
      {id} />
  </Remote>
</Remote>
