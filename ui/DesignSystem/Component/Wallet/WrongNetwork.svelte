<script lang="typescript">
  import { Emoji } from "../../Primitive";

  export let expectedNetwork = "Mainnet";
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
    gap: 1rem;

    width: 23.75rem;
    margin: 0 auto;
  }
</style>

<div class="wrapper">
  <div class="inner">
    <Emoji emoji="👛" size="huge" />
    <h2>Oops!</h2>
    <p class="typo-text">
      Your wallet is pointing to an unexpected network. Please, switch back to
      <span class="typo-text-bold">{expectedNetwork}</span>
      to keep using this feature.
    </p>
  </div>
</div>
