<script lang="ts">
  import { Button } from "../../../Primitive";
  import { Remote } from "../../../Component";

  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as p from "../../../../src/funding/pool";

  export let pool: p.Pool;

  const collectFunds = async (): Promise<void> => {
    try {
      await pool.collect();
      console.log("funds collected");
    } catch (error) {
      console.error("Failed to collect funds", error);
    }
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

  .item {
    display: inline-flex;
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
        <Button
          dataCy="collect-pool-button"
          variant="outline"
          on:click={collectFunds}>
          Collect your funds 🥳
        </Button>
      </div>
    </div>
  </div>
</Remote>
