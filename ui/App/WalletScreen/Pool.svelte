<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Remote from "ui/App/Remote.svelte";

  import Outgoing from "./Pool/Outgoing.svelte";
  import Incoming from "./Pool/Incoming.svelte";
  import CollectModal from "./Pool/CollectModal.svelte";

  import * as modal from "ui/src/modal";
  import * as fundingPool from "ui/src/funding/pool";
  import { TxKind, ongoing, store as txs } from "ui/src/transaction";
  import { store } from "ui/src/wallet";

  function onCollect() {
    fundingPool.store.set(pool);
    modal.toggle(CollectModal);
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
