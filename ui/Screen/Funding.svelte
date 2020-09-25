<script lang="ts">
  import Pool from "../DesignSystem/Component/Funding/Pool.svelte";

  // TODO(nuno): fetch these
  const amount = 99;
  const balance = 430;
  const enabled = false;
  const members = [`juliendonck`, `monadic`, `rudolfs`, `nakamoto`, `peterpan`];
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
  }
</style>

<div class="container">
  <Pool {amount} {balance} {enabled} {members} />
</div>
