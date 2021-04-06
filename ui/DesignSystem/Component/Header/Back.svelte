<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { Icon } from "../../Primitive";

  const dispatch = createEventDispatcher();
  const onArrowClick = () => {
    dispatch("arrowClick");
  };

  export let style: string = "";
</script>

<style>
  header {
    position: relative;
  }

  .back-button {
    position: absolute;
    left: -16px;
    cursor: pointer;
    transition: left 0.25s ease;
  }

  header:hover .back-button {
    left: -24px;
  }

  .back-button :global(svg) {
    fill: var(--color-foreground-level-2);
    transition: fill 0.25s ease;
  }

  header:hover .back-button :global(svg) {
    fill: var(--color-foreground-level-4);
  }

  /* `header` prefix is required to make this rule more specific than
   * the one above */
  header .back-button:hover :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<header {style}>
  <div class="back-button" on:click={() => onArrowClick()}>
    <Icon.ArrowLeft />
  </div>
  <slot />
</header>
