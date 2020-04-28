<script>
  import { format, transactions as store } from "../../../src/transaction.ts";

  import { Text } from "../../Primitive";
  import Remote from "../Remote.svelte";

  import Accordion from "./Accordion.svelte";
</script>

<Remote {store} let:data={transactions}>
  {#if transactions.length > 0}
    <Accordion
      transactions={transactions.map(format)}
      style="position: absolute; bottom: 32px; right: 32px;" />
  {/if}

  <div slot="error">
    <Text>Transactions errored</Text>
  </div>
</Remote>
