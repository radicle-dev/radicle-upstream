<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  const onArrowClick = () => {
    dispatch("arrowClick");
  };

  import { Icon } from "../../Primitive";

  export let style = null;
</script>

<style>
  header {
    position: relative;
  }

  header:hover .back-button {
    left: -24px;
  }

  header:hover :global(svg) {
    fill: var(--color-foreground-level-4);
  }

  .back-button {
    position: absolute;
    left: -16px;
    cursor: pointer;
    transition: left 0.25s ease;
  }

  .back-button :global(svg) {
    fill: var(--color-foreground-level-2);
    transition: fill 0.25s ease;
  }

  .back-button:hover :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<header {style}>
  <div class="back-button" on:click={() => onArrowClick()}>
    <Icon.ArrowLeft />
  </div>
  <slot />
</header>
