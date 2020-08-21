<script>
  import * as transaction from "../../../src/transaction.ts";

  import { Icon } from "../../Primitive";
  import TransactionSpinner from "./Spinner.svelte";

  import ItemDescription from "./ItemDescription.svelte";

  export let tx = null;
  export let accountId = null;

  $: message = transaction.formatMessage(tx.messages[0], accountId);
  $: progress = transaction.iconProgress(tx.state);
  $: iconState = transaction.iconState(tx.state);
</script>

<style>
  .item {
    align-items: center;
    background: var(--color-background);
    border-bottom: 1px solid var(--color-foreground-level-2);
    display: flex;
    justify-content: space-between;
    height: 56px;
  }

  .item:hover {
    background-color: var(--color-foreground-level-1);
  }

  .info {
    display: flex;
  }

  .icon {
    margin: 6px 12px 0;
  }

  .carret {
    display: flex;
    margin-right: 16px;
    vertical-align: middle;
  }
</style>

<div class="item" on:click data-cy="transaction-item">
  <div class="info">
    <div class="icon">
      <TransactionSpinner {progress} state={iconState} />
    </div>
    <ItemDescription {message} state={tx.state.type} />
  </div>
  <div class="carret">
    <Icon.Chevron />
  </div>
</div>
