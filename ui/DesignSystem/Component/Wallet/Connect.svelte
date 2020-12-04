<script lang="typescript">
  import { Illustration, Spinner } from "../../Component";
  import { Button } from "../../Primitive";

  import { Variant as IllustrationVariant } from "../../../src/illustration";

  export let onConnect: () => void;
  export let connecting = false;
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: var(--content-padding);

    height: 300px;
    width: 380px;
    margin: 20vh auto;
  }
</style>

<div class="wrapper">
  <Illustration variant={IllustrationVariant.Purse} />
  <p class="typo-text">
    In order to give and receive funds, you need to connect an Ethereum wallet
    you own.
  </p>
  {#if connecting}
    <Spinner />
  {:else}
    <Button disabled={connecting} on:click={onConnect}>
      Connect your wallet
    </Button>
  {/if}
</div>
