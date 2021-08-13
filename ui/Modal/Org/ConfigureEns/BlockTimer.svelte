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

  export let minAge: number;
  export let startBlock: number;
  export let onFinish: () => void;

  const walletStore = svelteStore.get(wallet.store);

  const requiredBlockCount = minAge + 1;
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
</script>

<style>
  p {
    color: var(--color-foreground-level-6);
  }
</style>

<p class="typo-text-bold">
  Confirmed {confirmedBlockCount} out of {requiredBlockCount}.
</p>
