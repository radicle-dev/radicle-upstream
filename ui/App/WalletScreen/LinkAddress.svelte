<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { lastClaimed } from "ui/src/attestation/lastClaimed";
  import { store as walletStore } from "ui/src/wallet";
  import * as modal from "ui/src/modal";

  import { Button, Emoji } from "ui/DesignSystem";
  import LinkAddressModal from "./LinkAddressModal.svelte";

  $: address = $walletStore.getAddress()?.toLowerCase();

  function onLink() {
    modal.toggle(LinkAddressModal);
  }
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 10vh 0;
    margin-top: 3.75rem;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
    width: 100%;
  }

  .inner {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    width: 23.75rem;
    margin: 0 auto;
  }

  p {
    margin-top: 1rem;
    margin-bottom: 1.25rem;
  }

  .spinner-wrapper {
    display: flex;
    align-items: center;
    gap: 10px;
  }
</style>

<div class="wrapper">
  <div class="inner">
    <Emoji emoji="👛" size="huge" />
    <p class="typo-text">
      In order to use Ethereum features, you need to link your Radicle ID and
      Ethereum address.
    </p>
    {#if !$lastClaimed || $lastClaimed !== address}
      <Button on:click={onLink} dataCy="link-button"
        >Link your Radicle ID</Button>
    {:else}
      <div class="spinner-wrapper">
        Linking your Radicle ID…
        <Button variant="transparent" on:click={onLink}>Retry</Button>
      </div>
    {/if}
  </div>
</div>
