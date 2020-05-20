<script>
  import { push } from "svelte-spa-router";

  import * as path from "../../../src/path.ts";

  import { Flex, Icon, Text } from "../../Primitive";

  // TODO(merle): Make component responsive to transactions change
  export let transactions = null;
  export let style = null;

  const stateToColor = {
    pending: "var(--color-caution)",
    success: "var(--color-positive)",
    error: "var(--color-negative)",
  };

  const stateToDescription = {
    pending: "Pending",
    success: "Success",
    error: "Error",
  };

  const stateToIconState = {
    pending: "caution",
    success: "positive",
    error: "negative",
  };

  const stateToSummary = {
    pending: "pending",
    success: "succeeded",
    error: "failed",
  };

  const summary = {
    pending: { count: 0, progress: 0 },
    success: { count: 0, progress: 0 },
    error: { count: 0, progress: 0 },
  };

  const fillSummary = () => {
    transactions.forEach((transaction) => {
      summary[transaction.state].count += 1;
      summary[transaction.state].progress += transaction.progress || 0;
    });
    summary["success"].progress = summary["success"].count * 100;
  };

  fillSummary();

  const progressSummary =
    summary["error"].count === transactions.length
      ? null
      : (summary["pending"].progress + summary["success"].progress) /
        transactions.length;

  const iconState =
    summary["error"].count > 0
      ? "negative"
      : summary["pending"].count > 0
      ? "caution"
      : "positive";

  const summaryState =
    summary["pending"].count > 0
      ? "pending"
      : summary["error"].count > 0
      ? "error"
      : "success";

  const summaryText = () => {
    if (summary[summaryState].count > 1) {
      return `${summary[summaryState].count} Transactions ${stateToSummary[summaryState]}`;
    } else {
      return `Transaction ${stateToSummary[summaryState]}`;
    }
  };

  let hidden = true;

  const hideCards = () => {
    hidden = !hidden;
  };
</script>

<style>
  .pipeline {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: var(--elevation-low);
    width: 274px;
    cursor: pointer;
    user-select: none;
  }

  .negative {
    border: 1px solid var(--color-negative);
  }

  .cards {
    background-color: var(--color-foreground-level-1);
    border-radius: 3px 3px 0 0;
  }

  .card {
    border-bottom: 1px solid var(--color-foreground-level-3);
    height: 64px;
  }

  .summary {
    height: 56px;
    background-color: var(--color-background);
    border-radius: 3px;
  }

  .description {
    align-self: center;
    display: flex;
    flex-direction: column;
  }

  .hidden {
    display: none;
  }
</style>

<div
  data-cy="transaction-center"
  class="pipeline"
  class:negative={iconState === 'negative'}
  {style}>
  <div class="cards" class:hidden>
    {#each transactions as transaction}
      <div
        class="card"
        on:click={() => {
          push(path.transactions(transaction.id));
        }}
        data-cy="card">
        <Flex style="height: 100%">
          <div slot="left" style="display: flex;">
            <Icon.TransactionState
              state={stateToIconState[transaction.state]}
              progress={transaction.progress}
              style="margin: 14px 14px 14px 18px;" />
            <div class="description">
              <Text variant="small" style="width: max-content;">
                {transaction.message}
              </Text>
              <Text
                variant="small"
                style="color: {stateToColor[transaction.state]}">
                {stateToDescription[transaction.state]}
              </Text>
            </div>
          </div>
          <div slot="right">
            <Icon.Chevron style="vertical-align: middle; margin-right: 16px;" />
          </div>
        </Flex>
      </div>
    {/each}
  </div>
  <div class="summary" on:click={hideCards}>
    <Flex>
      <div slot="left" style="display: flex;">
        <Icon.TransactionState
          style="margin: 12px 12px 12px 18px;"
          progress={progressSummary}
          state={iconState} />
        <Text variant="small" style="width: max-content; align-self: center;">
          {summaryText()}
        </Text>
      </div>
      <div slot="right">
        <Icon.Expand style="vertical-align: middle; margin-right: 16px;" />
      </div>
    </Flex>
  </div>
</div>
