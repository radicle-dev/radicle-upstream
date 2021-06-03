<script lang="typescript">
  import { Button, Emoji } from "ui/DesignSystem/Primitive";
  import Spinner from "ui/DesignSystem/Component/Spinner.svelte";
  import ModalLinkAddress from "../../Modal/Funding/LinkAddress.svelte";

  import * as modal from "../../src/modal";
  import { lastClaimed } from "../../src/attestation/lastClaimed";
  import { store as walletStore } from "../../src/wallet";

  $: address = $walletStore.account()?.address.toLowerCase();

  function onLink() {
    modal.toggle(ModalLinkAddress);
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
      In order to give and receive funds, you need to link your Radicle Identity
      to Ethereum.
    </p>
    {#if !$lastClaimed || $lastClaimed !== address}
      <Button on:click={onLink}>Link your ID</Button>
    {:else}
      <div class="spinner-wrapper">
        <Spinner height={24} width={24} />
        Linking your ID…
        <Button variant="transparent" on:click={onLink}>Retry</Button>
      </div>
    {/if}
  </div>
</div>
