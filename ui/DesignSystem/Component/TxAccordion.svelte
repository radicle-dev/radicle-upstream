<script>
  import { Flex, Icon, Text } from "../Primitive";
  import * as path from "../../lib/path.js";
  import { link } from "svelte-spa-router";
  // TODO: Make component responsive to transactions change
  export let transactions = null;
  export let style = null;

  const stateToColor = {
    pending: "var(--color-caution)",
    success: "var(--color-positive)",
    error: "var(--color-negative)"
  };

  const stateToDescription = {
    pending: "Pending",
    success: "Success",
    error: "Error"
  };

  const stateToIconState = {
    pending: "caution",
    success: "positive",
    error: "negative"
  };

  const stateToSummary = {
    pending: "pending",
    success: "succeeded",
    error: "failed"
  };

  const txSummary = {
    pending: { count: 0, progress: 0 },
    success: { count: 0, progress: 0 },
    error: { count: 0, progress: 0 }
  };

  const fillSummary = () => {
    transactions.forEach(tx => {
      txSummary[tx.state].count += 1;
      txSummary[tx.state].progress += tx.progress || 0;
    });
    txSummary["success"].progress = txSummary["success"].count * 100;
  };

  fillSummary();

  const progressSummary =
    txSummary["error"].count === transactions.length
      ? null
      : (txSummary["pending"].progress + txSummary["success"].progress) /
        transactions.length;

  const iconState =
    txSummary["error"].count > 0
      ? "negative"
      : txSummary["pending"].count > 0
      ? "caution"
      : "positive";

  const summaryState =
    txSummary["pending"].count > 0
      ? "pending"
      : txSummary["error"].count > 0
      ? "error"
      : "success";

  const summaryText = () => {
    if (txSummary[summaryState].count > 1) {
      return `${txSummary[summaryState].count} Transactions ${stateToSummary[summaryState]}`;
    } else {
      return `Transaction ${stateToSummary[summaryState]}`;
    }
  };

  let hidden = true;

  const hideCards = () => {
    hidden = hidden ? false : true;
  };
</script>

<style>
  .pipeline {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    box-shadow: 0px 4px 8px var(--color-foreground-level-3-opacity-08);
    width: 274px;
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

<div class="pipeline" class:negative={iconState === 'negative'} {style}>
  <div class="cards" class:hidden>
    {#each transactions as tx}
      <!-- TODO: Link card to tx detail view -->
      <div class="card">
        <Flex>
          <div slot="left">
            <Flex align="left">
              <Icon.TxState
                state={stateToIconState[tx.state]}
                progress={tx.progress}
                style="margin: 14px 14px 14px 18px;" />
              <div class="description">
                <Text variant="small" style="width: max-content;">
                  {tx.message}
                </Text>
                <Text variant="small" style="color: {stateToColor[tx.state]}">
                  {stateToDescription[tx.state]}
                </Text>
              </div>
            </Flex>
          </div>
          <div slot="right">
            <a href={path.transactions(tx.id)} use:link>
              <Icon.CarretBig
                style="vertical-align: middle; margin-right: 16px;" />
            </a>
          </div>
        </Flex>
      </div>
    {/each}
  </div>
  <div class="summary" on:click={hideCards}>
    <Flex>
      <div slot="left">
        <Flex align="left">
          <Icon.TxState
            style="margin: 12px 12px 12px 18px;"
            progress={progressSummary}
            state={iconState} />
          <Text variant="small" style="width: max-content; align-self: center;">
            {summaryText()}
          </Text>
        </Flex>
      </div>
      <div slot="right">
        <Icon.Source
          style="vertical-align: middle; transform: rotate(90deg); margin-right:
          16px;" />
      </div>
    </Flex>
  </div>
</div>
