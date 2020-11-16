<script lang="ts">
  import { Remote } from "../../../Component";

  import GetStarted from "./Outgoing/GetStarted.svelte";
  import Support from "./Outgoing/Support.svelte";

  import * as remote from "../../../../src/remote";
  import * as _pool from "../../../../src/funding/pool";

  export let pool: _pool.Pool;

  $: _pool.store.set(pool);

  let onboardingStatus = new _pool.OnboardingStatus();
  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      onboardingStatus = new _pool.OnboardingStatus(store.data);
    }
  });
</script>

<Remote store={pool.data} let:data={poolData}>
  {#if !onboardingStatus.isComplete()}
    <GetStarted bind:pool />
  {:else}
    <Support bind:pool />
  {/if}
</Remote>
