<script lang="typescript">
  import type { Tx } from "../../../../src/transaction";

  import { Icon } from "../../../Primitive";
  import TxSpinner from "../Spinner.svelte";

  import ItemDescription from "./ItemDescription.svelte";

  export let tx: Tx;
</script>

<style>
  .item {
    align-items: center;
    background: var(--color-background);
    border-bottom: 1px solid var(--color-foreground-level-2);
    display: flex;
    justify-content: space-between;
    height: 3.5rem;
  }

  .item:hover {
    background-color: var(--color-foreground-level-1);
  }

  .info {
    display: flex;
  }

  .icon {
    margin: 0.375rem 0.75rem 0;
  }

  .carret {
    display: flex;
    margin-right: 1rem;
    vertical-align: middle;
  }
</style>

<div class="item" on:click data-cy="transaction-item">
  <div class="info">
    <div class="icon">
      <TxSpinner status={tx.status} />
    </div>
    <ItemDescription {tx} />
  </div>
  <div class="carret">
    <Icon.ChevronRight />
  </div>
</div>
