<script lang="typescript">
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
    In order to give and receive funds, you need to connect an Ethereum wallet
    you own.
  </p>
  <Button disabled={connecting} on:click={onConnect}>
    Connect your wallet
  </Button>
</div>
