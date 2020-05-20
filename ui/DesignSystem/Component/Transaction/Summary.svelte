<script>
  import { Flex, Icon, Text } from "../../Primitive";

  import * as transaction from "../../../src/transaction.ts";

  export let summary = null;

  const progress = transaction.summaryIconProgress(summary);
  const rotate = transaction.summaryIconRotate(summary.counts);
  const state = transaction.summaryIconState(summary.counts);
  const text = transaction.summaryText(summary.counts);
</script>

<style>
  .summary {
    background-color: var(--color-background);
    border-radius: 3px;
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
        style="margin: 12px 12px 12px 18px;" />
      <Text
        style="align-self: center; text-transform: capitalize; width:
        max-content;"
        variant="small">
        {text}
      </Text>
    </div>
    <div slot="right">
      <Icon.Expand style="margin-right: 16px; vertical-align: middle;" />
    </div>
  </Flex>
</div>
