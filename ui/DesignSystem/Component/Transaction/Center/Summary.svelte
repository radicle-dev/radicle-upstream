<script lang="typescript">
  import { Icon } from "../../../Primitive";
  import TransactionSpinner from "../Spinner.svelte";

  import * as transaction from "../../../../src/transaction";

  export let counts: transaction.SummaryCounts;

  $: status = transaction.summaryStatus(counts);
  $: text = transaction.summaryText(counts);

</script>

<style>
  .summary {
    height: 3.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .text {
    display: flex;
  }

  .text p {
    align-self: center;
    width: max-content;
  }

  .icon {
    margin-right: 1rem;
  }

</style>

<div class="summary" on:click>
  <div class="text">
    <TransactionSpinner {status} style="margin: 0.75rem;" />
    <p class="typo-text-small-bold">{text}</p>
  </div>
  <div class="icon">
    <Icon.ChevronUpDown style="vertical-align: middle;" />
  </div>
</div>
