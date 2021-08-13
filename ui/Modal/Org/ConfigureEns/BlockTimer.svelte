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

  export let requiredBlocks = 5;
  export let startBlock: number = 0;
  export let onFinish: () => void = () => {};

  const walletStore = svelteStore.get(wallet.store);

  let strokeDashOffset: number = 300;

  const needToReachBlock = startBlock + requiredBlocks;

  const onBlock = (latestBlock: number) => {
    // Adding one more block to required blocks here just to be safe.
    const percentage = (needToReachBlock + 1 - latestBlock) / requiredBlocks;
    strokeDashOffset = 300 * percentage;

    if (needToReachBlock + 1 === latestBlock) {
      onFinish();
    }
  };

  walletStore.provider.on("block", onBlock);

  onDestroy(() => {
    walletStore.provider.off("block", onBlock);
  });
</script>

<style>
  .circle {
    transition: stroke-dashoffset 1s;
  }

  .circle-front {
    stroke: var(--color-primary);
  }

  .circle-back {
    stroke: var(--color-primary-level-2);
  }
</style>

<svg
  width="96"
  height="96"
  viewBox="0 0 96 96"
  fill="none"
  xmlns="http://www.w3.org/2000/svg">
  <circle
    style={`stroke-dasharray: 300; stroke-dashoffset: ${strokeDashOffset}; transform: rotate(-90deg); transform-origin: center;`}
    cx="48"
    cy="48"
    r="44"
    class="circle circle-front"
    stroke-linecap="round"
    stroke-width="8" />
  <circle cx="48" cy="48" r="44" class=".circle circle-back" stroke-width="8" />
</svg>
