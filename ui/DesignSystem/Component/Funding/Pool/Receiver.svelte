<script lang="ts">
  import { Remote, TxButton } from "../../../Component";

  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as _pool from "../../../../src/funding/pool";

  export let pool: _pool.Pool;

  const collectFunds = async (): Promise<void> => {
    await pool.collect();
  };
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

  header {
    width: 80%;
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
  }

  .item > * {
    margin: 0 5px;
  }
  .row + .row {
    border-top: solid 1px var(--color-foreground-level-2);
  }
</style>

<Remote store={pool.data} let:data={poolData}>
  <div class="incoming-container">
    <div class="row">
      <header>
        <h3>Incoming support</h3>
        <p>
          Funds from your supporters. Do you like money? Cash out to your
          connected wallet!
        </p>
      </header>
      <div class="item">
        <h3>{poolData.collectableFunds} DAI</h3>
        <TxButton
          disabled={poolData.collectableFunds <= 0}
          title={'Cash out'}
          onClick={collectFunds}
          variant={'primary'}
          successMessage={'Funds successfully collected'}
          errorMessage={e => `Failed to collect funds: ${e}`} />
      </div>
    </div>
  </div>
</Remote>
