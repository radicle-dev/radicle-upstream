<script lang="typescript">
  import { Button, Emoji } from "../../../../DesignSystem/Primitive";

  import * as modal from "../../../../src/modal";
  import * as path from "../../../../src/path";
  import * as fundingPool from "../../../../src/funding/pool";

  export let pool: fundingPool.Pool;

  function getStarted() {
    fundingPool.store.set(pool);
    modal.toggle(path.poolOnboarding());
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
    border-radius: 8px;
  }

  p {
    margin-top: 1rem;
    margin-bottom: 1.25rem;

    width: 300px;
  }
</style>

<div class="wrapper">
  <Emoji emoji="💸" size="huge" />
  <p class="typo-text">Stream digital money your favorite people.</p>
  <Button on:click={getStarted}>Get started</Button>
</div>
