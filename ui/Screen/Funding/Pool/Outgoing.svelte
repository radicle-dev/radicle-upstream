<script lang="ts">
  import { Remote } from "../../../DesignSystem/Component";

  import GetStarted from "./Outgoing/GetStarted.svelte";
  import Support from "./Outgoing/Support.svelte";

  import * as fundingPool from "../../../src/funding/pool";
  import { TxKind, ongoing } from "../../../src/transaction";
  import { store as txs } from "../../../src/transaction";

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
