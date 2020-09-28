<script lang="ts">
  import Pool from "../DesignSystem/Component/Funding/Pool.svelte";

  export let wallet: any;

  // TODO(nuno): fetch these
  const amount = 99;
  const balance = 430;
  const enabled = false;
  const members = "juliendonck, monadic, rudolfs, nakamoto, peterpan";
  const onFillUp = async (): Promise<void> => {
    await wallet.testTransfer(123);
  };
  const onDrain = (): Promise<void> => {
    console.log("onDrain");
    return wallet.testTransfer("Drain pool");
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
