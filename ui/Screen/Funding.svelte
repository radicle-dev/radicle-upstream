<script lang="ts">
  import Pool from "../DesignSystem/Component/Funding/Pool.svelte";

  // TODO(nuno): fetch these
  const amount = 99;
  const balance = 430;
  const enabled = false;
  const members = [`juliendonck`, `monadic`, `rudolfs`, `nakamoto`, `peterpan`];
  const onFillUp = (): Promise<void> => {
    window.alert("FillUp");
    return Promise.resolve();
  };
  const onDrain = (): Promise<void> => {
    window.alert("Drain");
    return Promise.resolve();
  };
</script>

<style>
  .container {
    min-width: var(--content-min-width);
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
  }
</style>

<div class="container">
  <Pool {amount} {balance} {enabled} {members} {onFillUp} {onDrain} />
</div>
