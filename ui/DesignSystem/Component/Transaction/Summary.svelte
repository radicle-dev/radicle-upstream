<script>
  import { Flex, Icon } from "../../Primitive";
  import TransactionSpinner from "./Spinner.svelte";

  import * as transaction from "../../../src/transaction.ts";

  export let summary = null;

  $: progress = transaction.summaryIconProgress(summary);
  $: rotate = transaction.summaryIconRotate(summary.counts);
  $: state = transaction.summaryIconState(summary.counts);
  $: text = transaction.summaryText(summary.counts);
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
      <TransactionSpinner
        {progress}
        {rotate}
        {state}
        style="margin: 11px 12px;" />
      <p class="typo-text-small-bold">{text}</p>
    </div>
    <div slot="right">
      <Icon.Expand style="margin-right: 16px; vertical-align: middle;" />
    </div>
  </Flex>
</div>
