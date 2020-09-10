<script>
  import { createEventDispatcher } from "svelte";

  import { RadicleLogo } from "../../DesignSystem/Component";
  import { Button } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  let ran = false;

  const onKeydown = event => {
    if (!ran && event.key === "Enter") {
      next();
      ran = true;
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

  p {
    text-align: center;
    color: var(--color-foreground-level-6);
    margin: 2.5rem 0;
    max-width: 20rem;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container" data-cy="welcome-screen">
  <RadicleLogo />

  <p>A free and open-source way to host, share, and build software together.</p>

  <Button
    style="flex-shrink: 0; margin-bottom: 1.5rem;"
    on:click={next}
    dataCy="get-started-button">
    Get started
  </Button>
</div>
