<script lang="ts">
  import { push } from "svelte-spa-router";
  import * as path from "../src/path";

  import Pool from "../DesignSystem/Component/Funding/Pool.svelte";
  import * as pool from "../src/funding/pool";

  // TODO(nuno): fetch these
  const amount = 99;
  const balance = 430;
  const enabled = false;
  const members = "juliendonck, monadic, rudolfs, nakamoto, peterpan";

  export let wallet: any;

  const openSendModal = () => {
    push(path.sendFunds());
  };

  const onFillUp = async (): Promise<void> => {
    const tx: pool.Transaction = {
      context: "Fill up your pool 😉",
      from: "0x789", // TOOD(nuno): use right address
      to: "0x123000", // TOOD(nuno): use contract address?
      onConfirmed: value => {
        return wallet!.testTransfer(value);
      },
    };
    pool.txStore.set(tx);
    openSendModal();
  };
  const onDrain = (): Promise<void> => {
    console.log("onDrain");
    return wallet!.testTransfer(10);
  };

  const onSave = (): Promise<void> => {
    console.log("onSave");
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
  <Pool {amount} {balance} {enabled} {members} {onFillUp} {onDrain} {onSave} />
</div>
