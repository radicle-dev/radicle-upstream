<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  import { RadicleLogo } from "../../DesignSystem/Component";
  import { Button } from "../../DesignSystem/Primitive";

  const onKeydown = event => {
    if (event.key === "Enter") {
      next();
    }
  };

  const next = () => {
    dispatch("next");
  };
</script>

<style>
  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }

  .container p {
    text-align: center;
    color: var(--color-foreground-level-5);
    margin: 2.5rem 0;
    max-width: 20rem;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container">
  <RadicleLogo />

  <p>A free and open-source way to host, share, and build software together.</p>

  <Button
    style="flex-shrink: 0; margin-bottom: 24px;"
    on:click={next}
    dataCy="get-started-button">
    Get started
  </Button>
</div>
