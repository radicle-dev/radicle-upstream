<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Remote from "ui/App/Remote.svelte";

  import GetStarted from "./Outgoing/GetStarted.svelte";
  import Support from "./Outgoing/Support.svelte";

  import * as fundingPool from "ui/src/funding/pool";
  import { TxKind, ongoing } from "ui/src/transaction";
  import { store as txs } from "ui/src/transaction";

  export let pool: fundingPool.Pool;

  let ongoingOnboardingTx = false;
  $: ongoingOnboardingTx = $txs.some(ongoing(TxKind.SupportOnboarding));
</script>

<Remote store={pool.data} let:data={poolData}>
  {#if fundingPool.isOnboarded(poolData)}
    <Support bind:pool />
  {:else}
    <GetStarted ongoingTx={ongoingOnboardingTx} bind:pool />
  {/if}
</Remote>
