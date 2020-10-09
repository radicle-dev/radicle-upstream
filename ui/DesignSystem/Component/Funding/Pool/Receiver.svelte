<script lang="ts">
  import { Remote, StatefulButton } from "../../../Component";

  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as _pool from "../../../../src/funding/pool";

  export let pool: _pool.Pool;

  const collectFunds = async (): Promise<void> => {
    await pool.collect();
  };
</script>

<style>
  .content {
    margin: 2rem 0;
  }

  .row {
    padding: 1.75rem 0px;
    display: flex;
    justify-content: space-between;
  }

  header {
    width: 80%;
  }

  .item > * {
    margin: 0 5px;
  }
  .row + .row {
    border-top: solid 1px var(--color-foreground-level-2);
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="content">
    <h3>Collect</h3>

    <div class="row">
      <header>
        <p>
          Hi there! You've got
          <currency>DAI</currency>
          {poolData.collectableFunds} waiting to be collected. This is the sum of
          all the donations you are eligible to receive.
        </p>
      </header>
      <div class="item">
        <StatefulButton
          title={'Collect your funds 🥳'}
          onClick={collectFunds}
          variant={'outline'}
          successMessage={'✓ Funds successfully collected'}
          errorMessage={e => `Failed to collect funds: ${e}`} />
      </div>
    </div>
  </div>
</Remote>
