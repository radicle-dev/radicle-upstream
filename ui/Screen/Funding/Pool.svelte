<script lang="ts">
  import Outgoing from "../../DesignSystem/Component/Funding/Pool/Outgoing.svelte";
  import Incoming from "../../DesignSystem/Component/Funding/Pool/Incoming.svelte";

  import type { Pool } from "../../src/funding/pool";

  export let pool: Pool;
</script>

<style>
  .container div + div {
    margin-top: var(--content-padding);
  }
</style>

<div class="pool-container">
  <Incoming {pool} />
  <Outgoing {pool} />
</div>
