<script lang="ts">
  import Giver from "../DesignSystem/Component/Funding/Pool/Giver.svelte";
  import Receiver from "../DesignSystem/Component/Funding/Pool/Receiver.svelte";

  import * as _pool from "../src/funding/pool";

  export let wallet: any;

  const pool = _pool.make(wallet);
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
  }

  .container div + div {
    margin-top: var(--content-padding);
  }
</style>

<div class="container">
  <Receiver {pool} />
  <Giver {pool} />
</div>
