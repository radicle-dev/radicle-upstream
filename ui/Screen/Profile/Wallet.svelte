<script>
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";
</script>

<Remote {store} let:data={transactions}>
  <Wallet {transactions} />
</Remote>
