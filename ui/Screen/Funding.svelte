<script lang="ts">
  import Pool from "../DesignSystem/Component/Funding/Pool.svelte";
  import * as pool from "../src/funding/pool";

  export let wallet: any;

  function makePool(): pool.Pool {
    return pool.make(wallet);
  }
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
  }
</style>

<div class="container">
  <!-- TODO(nuno): Collect pool funds screen goes here -->
  <Pool pool={makePool()} />
</div>
