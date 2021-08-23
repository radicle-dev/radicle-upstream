<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onMount, onDestroy } from "svelte";

  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  export let minimumCommitmentAge: number;
  export let txHash: string;
  export let onFinish: () => void;

  const walletStore = svelteStore.get(wallet.store);

  const requiredBlockCount = minimumCommitmentAge + 1;
  let startBlock: number;
  let confirmedBlockCount: number = 0;
  let done: boolean = false;

  const onBlock = (currentBlock: number) => {
    confirmedBlockCount = currentBlock - startBlock;

    if (!done && confirmedBlockCount >= requiredBlockCount) {
      done = true;
      onFinish();
    }
  };

  walletStore.provider.on("block", onBlock);

  onDestroy(() => {
    walletStore.provider.off("block", onBlock);
  });

  onMount(async () => {
    const tx = await walletStore.provider.getTransaction(txHash);
    const block = await walletStore.provider.getBlockNumber();

    // If the block has not been mined, `blockNumber` is null.
    if (tx.blockNumber) {
      startBlock = tx.blockNumber;
    } else {
      startBlock = block;
    }
    onBlock(block);
  });
</script>

<style>
  p {
    color: var(--color-foreground-level-6);
  }
</style>

<p class="typo-text-bold">
  Confirmed {confirmedBlockCount} out of {requiredBlockCount}.
</p>
