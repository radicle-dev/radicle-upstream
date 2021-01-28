<script lang="ts">
  import { Remote } from "../../DesignSystem/Component";

  import Outgoing from "./Pool/Outgoing.svelte";
  import Incoming from "./Pool/Incoming.svelte";

  import * as modal from "../../src/modal";
  import * as fundingPool from "../../src/funding/pool";
  import type { Pool } from "../../src/funding/pool";
  import * as path from "../../src/path";
  import * as transaction from "../../src/transaction";

  export let pool: Pool;

  function onCollect() {
    fundingPool.store.set(pool);
    modal.toggle(path.collectFunds());
  }

  let ongoingCollect = false;
  transaction.store.subscribe(_ => {
    ongoingCollect = transaction.ongoing(transaction.TxKind.CollectFunds);
  });
</script>

<style>
  .pool-container {
    width: 100%;
  }
</style>

<div class="pool-container">
  <Remote store={pool.data} let:data={poolData}>
    {#if poolData.collectableFunds.gt(0)}
      <Incoming
        amount={poolData.collectableFunds}
        {onCollect}
        {ongoingCollect}
        style={'margin-bottom: var(--content-padding)'} />
    {/if}
  </Remote>
  <Outgoing {pool} />
</div>
