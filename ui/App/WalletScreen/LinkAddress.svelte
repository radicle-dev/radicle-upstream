<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import {
    selectedEnvironment as ethereumEnvironment,
    supportedNetwork,
  } from "ui/src/ethereum";
  import {
    attestationStatus,
    AttestationStatus,
  } from "ui/src/attestation/status";
  import { lastClaimed } from "ui/src/attestation/lastClaimed";
  import { store as walletStore } from "ui/src/wallet";
  import * as modal from "ui/src/modal";
  import { store, Status } from "ui/src/wallet";

  import Button from "design-system/Button.svelte";
  import Emoji from "design-system/Emoji.svelte";

  import LinkAddressModal from "./LinkAddressModal.svelte";

  $: address = $walletStore.getAddress()?.toLowerCase();

  function onLink(): void {
    modal.toggle(LinkAddressModal);
  }

  $: wallet = $store;
  $: w = $wallet;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 3rem 0;
    margin-top: 1.5rem;

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
    margin: 0 auto;
  }

  p {
    margin: 1rem 1rem 1.25rem 1rem;
  }

  .spinner-wrapper {
    display: flex;
    align-items: center;
    gap: 10px;
  }
</style>

{#if w.status === Status.Connected}
  {#if supportedNetwork($ethereumEnvironment) === w.connected.network}
    <div class="wrapper">
      <div class="inner">
        {#if $attestationStatus === AttestationStatus.Fetching}
          <Emoji emoji="🧦" size="huge" />
          <p class="typo-text">
            Checking whether your Radicle identity and Ethereum address are
            linked…
          </p>
        {:else if $attestationStatus === AttestationStatus.Valid}
          <Emoji emoji="🧦" size="huge" />
          <p class="typo-text">
            Your Radicle identity and Ethereum address are linked.
          </p>
        {:else}
          <Emoji emoji="👛" size="huge" />
          <p class="typo-text">
            To use Ethereum features, you need to link your Radicle identity and
            Ethereum address.
          </p>
          {#if !$lastClaimed || $lastClaimed !== address}
            <Button on:click={onLink} dataCy="link-button"
              >Link your Radicle identity</Button>
          {:else}
            <div class="spinner-wrapper">
              Linking your Radicle identity…
              <Button variant="transparent" on:click={onLink}>Retry</Button>
            </div>
          {/if}
        {/if}
      </div>
    </div>
  {/if}
{/if}
