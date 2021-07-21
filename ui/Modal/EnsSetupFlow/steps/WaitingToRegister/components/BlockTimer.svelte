<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as wallet from "ui/src/wallet";
  import * as svelteStore from "ui/src/svelteStore";
  import { onDestroy } from "svelte";

  const walletStore = svelteStore.get(wallet.store);

  export let requiredBlocks = 5;
  export let startBlock: number = 0;
  export let onFinish: () => void = () => {};

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

  .circleFront {
    stroke: var(--color-primary);
  }

  .circleBack {
    stroke: var(--color-primary-level-2);
  }
</style>

<div id="wrapper">
  <svg
    width="96"
    height="96"
    viewBox="0 0 96 96"
    fill="none"
    xmlns="http://www.w3.org/2000/svg">
    <circle
      style="stroke-dasharray: 300; stroke-dashoffset: {strokeDashOffset}; transform: rotate(-90deg); transform-origin: center;"
      cx="48"
      cy="48"
      r="44"
      class="circle circleFront"
      stroke-linecap="round"
      stroke-width="8" />
    <circle
      cx="48"
      cy="48"
      r="44"
      class=".circle circleBack"
      stroke-width="8" />
  </svg>
</div>
