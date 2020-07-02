<script>
  import { Flex, Icon, Text } from "../../Primitive";

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
</style>

<div class="summary" on:click>
  <Flex>
    <div slot="left" class="text">
      <Icon.TransactionState
        {progress}
        {rotate}
        {state}
        style="margin: 11px 12px;" />
      <Text style="align-self: center; width: max-content;" variant="small">
        {text}
      </Text>
    </div>
    <div slot="right">
      <Icon.Expand style="margin-right: 16px; vertical-align: middle;" />
    </div>
  </Flex>
</div>
