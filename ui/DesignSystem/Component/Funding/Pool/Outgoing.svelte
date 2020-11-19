<script lang="ts">
  import { Remote } from "../../../Component";

  import GetStarted from "./Outgoing/GetStarted.svelte";
  import Support from "./Outgoing/Support.svelte";

  import * as _pool from "../../../../src/funding/pool";

  export let pool: _pool.Pool;
</script>

<Remote store={pool.data} let:data={poolData}>
  {#if _pool.isOnboarded(poolData)}
    <Support bind:pool />
  {:else}
    <GetStarted bind:pool />
  {/if}
</Remote>
