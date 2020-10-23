<script lang="typescript">
  import { Flex, Icon } from "../../Primitive";
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
  <Flex>
    <div slot="left" class="text">
      <TransactionSpinner {status} style="margin: 11px 12px;" />
      <p class="typo-text-small-bold">{text}</p>
    </div>
    <div slot="right">
      <Icon.ChevronUpDown style="margin-right: 16px; vertical-align: middle;" />
    </div>
  </Flex>
</div>
