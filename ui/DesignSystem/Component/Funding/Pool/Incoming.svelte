<script lang="typescript">
  import { Remote } from "../../../Component";
  import { Button, Icon } from "../../../Primitive";

  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";
  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as _pool from "../../../../src/funding/pool";
  import * as transaction from "../../../../src/transaction";

  export let pool: _pool.Pool;

  const collectFunds = async (): Promise<void> => {
    _pool.store.set(pool);
    modal.toggle(path.collectFunds());
  };

  let ongoingCollect = false;

  transaction.store.subscribe(_ => {
    ongoingCollect = transaction.ongoing(transaction.TxKind.CollectFunds);
  });
</script>

<style>
  .incoming-container {
    margin: 2rem 0;
    padding: var(--content-padding);
    padding-top: calc(1.2 * var(--content-padding));

    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;
  }

  .row {
    display: flex;
    justify-content: space-between;
  }

  h3,
  p {
    color: #546474;
  }

  header h3 + p {
    margin-top: 10px;
  }

  .item {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    min-width: 200px;
  }

  .item > * + * {
    margin-left: 7px;
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="incoming-container">
    <div class="row">
      <header>
        <h3>Incoming support</h3>
        <p>Funds from your supporters will show up here.</p>
      </header>
      <div class="item">
        <p>Balance</p>
        <h3 style="display: flex; margin-left: 10px">
          <Icon.CurrencyDAI
            style="fill: var(--color-foreground-level-6); padding-top: 3px;" />
          {poolData.collectableFunds}
        </h3>
        {#if poolData.collectableFunds > 0 && !ongoingCollect}
          <Button
            style="margin-left: 10px;"
            disabled={ongoingCollect}
            on:click={collectFunds}
            variant={'primary'}>
            Cash out
          </Button>
        {/if}
      </div>
    </div>
  </div>
</Remote>
