<script lang="typescript">
  import { Dai, Remote } from "../../../Component";
  import { Button } from "../../../Primitive";

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
    padding: calc(var(--content-padding) / 2);

    border: 1px solid #ebeff3;
    box-sizing: border-box;
    border-radius: 8px;
    background-color: var(--color-secondary-level-1);
  }

  .row {
    display: flex;
    justify-content: space-between;
  }

  .text {
    display: flex;
    align-items: center;
    color: var(--color-secondary);
  }

  .item > * + * {
    margin-left: 7px;
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="incoming-container">
    <div class="row">
      <div class="text">
        There’s
        <div class="typo-text-bold">
          <Dai color={'var(--color-secondary)'} style="margin-right: 5px">
            {poolData.collectableFunds}
          </Dai>
        </div>
        waiting on you from supporters.
      </div>

      <Button
        style="margin-left: 12px;"
        disabled={ongoingCollect}
        on:click={collectFunds}
        variant={'secondary'}>
        Cash out
      </Button>
    </div>
  </div>
</Remote>
