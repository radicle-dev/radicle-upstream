<script lang="typescript">
  import { ethereumAddress } from "../../../src/identity";
  import { ellipsed } from "../../../src/style";

  import { Button, Emoji } from "../../Primitive";

  export let onConnect: () => void;
  export let connecting = false;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: calc(100vh - var(--bigheader-height));
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
    {#if $ethereumAddress === null}
      In order to give and receive funds, you need to connect an Ethereum wallet
      you own.
    {:else}
      In order to give and receive funds, you need to connect your linked
      Ethereum account
      {ellipsed($ethereumAddress)}.
    {/if}
  </p>
  <Button disabled={connecting} on:click={onConnect}>
    Connect your wallet
  </Button>
</div>
