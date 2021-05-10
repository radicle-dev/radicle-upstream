<script lang="typescript">
  import { Button, Emoji } from "../../DesignSystem/Primitive";
  import ModalLinkAddress from "../../Modal/Funding/LinkAddress.svelte";

  import * as modal from "../../src/modal";

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

</style>

<div class="wrapper">
  <div class="inner">
    <Emoji emoji="👛" size="huge" />
    <p class="typo-text">
      In order to give and receive funds, you need to link your Radicle Identity
      to Ethereum.
    </p>
    <Button on:click={onLink}>Link your ID</Button>
  </div>
</div>
