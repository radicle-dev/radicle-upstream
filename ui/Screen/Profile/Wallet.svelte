<script>
  import { transactions as store } from "../../src/transaction.ts";
  import { Wallet, Remote } from "../../DesignSystem/Component";

  const userTransactions = transactions => {
    return transactions.filter(tx => {
      return tx.messages[0].domainType !== "org";
    });
  };
</script>

<Remote {store} let:data={transactions}>
  <Wallet
    dataCy="user-wallet"
    transactions={userTransactions(transactions)}
    balance="342"
    accountId="hyda4fhdx8up8hhocagkdz3d41txb98stw4pkqe3uommo1p5m6s9oy" />
</Remote>
