<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onDestroy } from "svelte";

  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  export let commitmentBlock: number;
  export let minimumCommitmentAge: number;
  export let onFinish: () => void;

  const walletStore = svelteStore.get(wallet.store);

  // There seems to be an off-by-one error in the contract, because if we don't
  // wait for that one extra block we get an error saying that the commitment
  // isn't old enough.
  const requiredBlockCount = minimumCommitmentAge + 1;
  let confirmedBlockCount: number = 0;

  const onBlock = (currentBlock: number) => {
    confirmedBlockCount = currentBlock - commitmentBlock;

    if (confirmedBlockCount >= requiredBlockCount) {
      onFinish();
      walletStore.provider.off("block", onBlock);
      // When we resume a saved commitment, it can happen that more blocks have
      // been included than the minimum required amount in the mean time.
      // We don't want to overflow the counter that we show to the user.
      confirmedBlockCount = requiredBlockCount;
    }
  };

  walletStore.provider.on("block", onBlock);

  onDestroy(() => {
    walletStore.provider.off("block", onBlock);
  });
</script>

<style>
  p {
    color: var(--color-foreground-level-6);
  }
</style>

<p class="typo-text-bold">
  Confirmed {confirmedBlockCount} out of {requiredBlockCount} blocks.
</p>
