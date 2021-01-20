<script lang="ts">
  import { Remote } from "../../../DesignSystem/Component";

  import GetStarted from "./Outgoing/GetStarted.svelte";
  import Support from "./Outgoing/Support.svelte";

  import * as fundingPool from "../../../src/funding/pool";

  export let pool: fundingPool.Pool;
</script>

<Remote store={pool.data} let:data={poolData}>
  {#if fundingPool.isOnboarded(poolData)}
    <Support bind:pool />
  {:else}
    <GetStarted bind:pool />
  {/if}
</Remote>
