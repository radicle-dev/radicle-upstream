<script lang="ts">
  import Outgoing from "../../DesignSystem/Component/Funding/Pool/Outgoing.svelte";
  import Incoming from "../../DesignSystem/Component/Funding/Pool/Incoming.svelte";

  import type { Pool } from "../../src/funding/pool";

  export let pool: Pool;
</script>

<div class="pool-container">
  <Incoming {pool} />
  <Outgoing {pool} />
</div>
