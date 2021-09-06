<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Button from "ui/DesignSystem/Button.svelte";
  import Emoji from "ui/DesignSystem/Emoji.svelte";

  export let onConnect: () => void;
  export let connecting = false;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    text-align: center;
    max-width: 30rem;
    margin: 0 auto;
    gap: 1.5rem;
  }

  .connecting {
    cursor: wait;
  }
</style>

<div class="wrapper" class:connecting>
  <Emoji emoji="👛" size="huge" />
  <p class="typo-text">
    In order to use our funding features, you need to connect an Ethereum
    wallet.
  </p>
  <Button disabled={connecting} on:click={onConnect} dataCy="connect-wallet">
    Connect your wallet
  </Button>
</div>
