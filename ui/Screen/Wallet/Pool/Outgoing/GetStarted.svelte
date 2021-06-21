<script lang="typescript">
  import { Button, Emoji } from "ui/DesignSystem";
  import ModalPoolOnboarding from "../../../../Modal/Funding/Onboarding.svelte";

  import * as modal from "ui/src/modal";
  import * as fundingPool from "ui/src/funding/pool";

  export let pool: fundingPool.Pool;
  // Whether the onboarding tx is ongoing and this screen
  // should, therefore, be disabled.
  export let ongoingTx = false;

  function getStarted() {
    fundingPool.store.set(pool);
    modal.toggle(ModalPoolOnboarding);
  }
</script>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    text-align: center;
    padding: 15vh 0;

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 0.5rem;
  }

  p {
    margin-top: 1rem;
    margin-bottom: 1.25rem;

    width: 18.75rem;
  }
</style>

<div class="wrapper">
  <Emoji emoji="💸" size="huge" />
  <p class="typo-text">Stream digital money to your favorite people.</p>
  <Button disabled={ongoingTx} on:click={getStarted}>Get started</Button>
</div>
