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

  .merge-request-page:hover header .back-button {
    left: -24px;
  }

  .merge-request-page:hover header .back-button :global(svg) {
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

  .merge-request-page:hover header .back-button:hover :global(svg) {
    fill: var(--color-foreground-level-6);
  }
</style>

<header {style}>
  <div class="back-button" on:click={() => onArrowClick()}>
    <Icon.ArrowLeft />
  </div>
  <slot />
</header>
