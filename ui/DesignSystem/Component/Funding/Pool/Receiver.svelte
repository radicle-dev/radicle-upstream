<script lang="ts">
  import { Button, Icon } from "../../../Primitive";
  import { Remote, Spinner } from "../../../Component";

  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as _pool from "../../../../src/funding/pool";
  import * as notification from "../../../../src/notification";

  export let pool: _pool.Pool;

  enum Status {
    Idle,
    Collecting,
    Succeeded,
    Failed,
  }

  let status = Status.Idle;

  // Set the status to a new value. Wait 1 second before considered
  // done to smooth the status transitions in the UI.
  function setStatus(newStatus: Status): Promise<void> {
    status = newStatus;
    return continueAfter(1);
  }

  const collectFunds = async (): Promise<void> => {
    try {
      await setStatus(Status.Collecting);
      await pool.collect();
      await setStatus(Status.Succeeded);
    } catch (error) {
      notification.error(`Failed to collect funds: ${error}`);
      await setStatus(Status.Failed);
    } finally {
      await setStatus(Status.Idle);
    }
  };

  function continueAfter(seconds: number): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve();
      }, seconds * 1000);
    });
  }
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

  .item {
    display: inline-flex;
    align-items: center;
    /* Having a min-height helps the UI staying fixed when switching statuses.*/
    min-height: 40px;
    /* Having a min-width helps the UI having all the different statuses horizontally aligned */
    min-width: 100px;
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
          Hi there! You've got ETH {poolData.collectableFunds} waiting to be collected.
          This is the sum of all the donations you are eligible to receive.
        </p>
      </header>
      <div class="item">
        {#if status === Status.Idle}
          <Button
            dataCy="collect-pool-button"
            variant="outline"
            on:click={collectFunds}>
            Collect your funds 🥳
          </Button>
        {:else if status === Status.Collecting}
          <Spinner />
        {:else if status === Status.Succeeded}
          <Icon.CheckCircle style={`fill: var(--color-positive)`} />
        {:else if status === Status.Failed}
          <Icon.CrossCircle style={`fill: var(--color-negative)}`} />
        {/if}
      </div>
    </div>
  </div>
</Remote>
