<script lang="typescript">
  import { Illustration, Spinner } from "../../Component";
  import { Button } from "../../Primitive";

  import { Variant as IllustrationVariant } from "../../../src/illustration";
  import { linkedAddress } from "../../../src/identity";
  import { displayAddress } from "../../../src/funding/pool";

  export let onConnect: () => void;
  export let connecting = false;

  const text = $linkedAddress
    ? `You’ve linked your Radicle ID to Ethereum account ${displayAddress(
        $linkedAddress
      )}, but your wallet is not connected.`
    : "In order to give and receive funds, you need to link your Radicle Identity to Ethereum.";

  const button = $linkedAddress ? "Connect your wallet" : "Link your ID";
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
  <p class="typo-text">{text}</p>
  {#if connecting}
    <Spinner />
  {:else}
    <Button disabled={connecting} on:click={onConnect}>{button}</Button>
  {/if}
</div>
