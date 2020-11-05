<script lang="typescript">
  import { Icon } from "../../Primitive";
  import TransactionSpinner from "./Spinner.svelte";

  import * as transaction from "../../../src/transaction.ts";

  export let counts: transaction.SummaryCounts;

  $: status = transaction.summaryStatus(counts);
  $: text = transaction.summaryText(counts);
</script>

<style>
  .summary {
    height: 56px;
  }
  .text {
    display: flex;
  }
  .text p {
    align-self: center;
    width: max-content;
  }
</style>

<div class="summary" on:click>
  <div class="text">
    <TransactionSpinner {status} style="margin: 11px 12px;" />
    <p class="typo-text-small-bold">{text}</p>
  </div>
  <div class="icon">
    <Icon.ChevronUpDown style="margin-right: 16px; vertical-align: middle;" />
  </div>
</div>
