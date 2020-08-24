<script>
  import { Icon } from "../../Primitive";

  export let disabled = false;
</script>

<style>
  .button {
    cursor: pointer;
  }
  .button:hover {
    fill: var(--color-secondary);
  }
  .button:hover :global(svg) {
    fill: var(--color-secondary);
  }

  .button.disabled:hover {
    fill: var(--color-foreground-level-3);
    cursor: not-allowed;
  }

  .button.disabled :global(svg) {
    fill: var(--color-foreground-level-3);
  }
</style>

<svg
  class="button"
  class:disabled
  data-cy="add-org-button"
  on:click
  width="32"
  height="32"
  fill="var(--color-foreground-level-3)"
  viewBox="0 0 32 32"
  xmlns="http://www.w3.org/2000/svg">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M28 1H4C2.34315 1 1 2.34315 1 4V28C1 29.6569 2.34315 31 4 31H28C29.6569
    31 31 29.6569 31 28V4C31 2.34315 29.6569 1 28 1ZM4 0C1.79086 0 0 1.79086 0
    4V28C0 30.2091 1.79086 32 4 32H28C30.2091 32 32 30.2091 32 28V4C32 1.79086
    30.2091 0 28 0H4Z" />
  <g transform="translate(4,4)">
    <Icon.PlusBig />
  </g>
</svg>
