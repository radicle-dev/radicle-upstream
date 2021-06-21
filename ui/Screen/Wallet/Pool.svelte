<script lang="ts">
  import { Remote } from "ui/DesignSystem";

  import Outgoing from "./Pool/Outgoing.svelte";
  import Incoming from "./Pool/Incoming.svelte";
  import ModalCollect from "ui/Modal/Funding/Pool/Collect.svelte";

  import * as modal from "ui/src/modal";
  import * as fundingPool from "ui/src/funding/pool";
  import { TxKind, ongoing, store as txs } from "ui/src/transaction";
  import { store } from "ui/src/wallet";

  function onCollect() {
    fundingPool.store.set(pool);
    modal.toggle(ModalCollect);
  }

  let ongoingCollect = false;
  $: ongoingCollect = $txs.some(ongoing(TxKind.CollectFunds));
  $: wallet = $store;
  $: pool = fundingPool.make(wallet);
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
        style={"margin-bottom: var(--content-padding)"} />
    {/if}
  </Remote>
  <Outgoing {pool} />
</div>
