<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as router from "ui/src/router";
  import Button from "ui/DesignSystem/Button.svelte";
  import Emoji from "ui/DesignSystem/Emoji.svelte";

  export let expectedNetwork = "Mainnet";
  export let walletNetwork: string;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 10vh 0;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
    width: 100%;

    margin-top: 3.75rem;
  }

  .inner {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    gap: 1rem;

    width: 23.75rem;
    margin: 0 auto;
  }
</style>

<div class="wrapper">
  <div class="inner">
    <Emoji emoji="🥀" size="huge" />
    <h2>Something’s not right…</h2>
    <p class="typo-text">
      Your wallet is on <span class="typo-text-bold">{walletNetwork}</span>, but
      Upstream is on <span class="typo-text-bold">{expectedNetwork}</span>. Make
      sure both your wallet and Upstream are set to the same network.
    </p>
    <Button on:click={() => router.push({ type: "settings" })}
      >Go to settings</Button>
  </div>
</div>
