<script>
  import { Icon } from "../../Primitive";

  export let disabled = false;
</script>

<style>
  .button {
    cursor: pointer;
  }

  .button.disabled:hover {
    fill: var(--color-foreground-level-3) !important;
  }

  .button.disabled :global(svg) {
    fill: var(--color-foreground-level-3) !important;
  }
</style>

<svg
  class="button"
  class:disabled
  data-cy="add-org-button"
  on:click
  width="36"
  height="36"
  fill="var(--color-foreground-level-3)"
  viewBox="0 0 36 36"
  xmlns="http://www.w3.org/2000/svg">
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M1.23442 35.8482L1.61748 34.9245C1.734 34.9728 1.86257 35 2
    35H4V36H2C1.72882 36 1.47024 35.946 1.23442 35.8482ZM32 36V35H34C34.1374 35
    34.266 34.9728 34.3825 34.9245L34.7656 35.8482C34.5298 35.946 34.2712 36 34
    36H32ZM36 4H35V2C35 1.86257 34.9728 1.734 34.9245 1.61748L35.8482
    1.23442C35.946 1.47024 36 1.72882 36 2V4ZM4 0H2C1.72882 0 1.47024 0.0539723
    1.23442 0.151761L1.61748 1.07549C1.734 1.02717 1.86257 1 2 1H4V0ZM0
    32H1V34C1 34.1374 1.02717 34.266 1.07549 34.3825L0.151761 34.7656C0.0539724
    34.5298 0 34.2712 0 34V32ZM0 28H1V24H0V28ZM0 20H1V16H0V20ZM0 12H1V8H0V12ZM0
    4H1V2C1 1.86257 1.02717 1.734 1.07549 1.61748L0.151761 1.23443C0.0539723
    1.47024 0 1.72882 0 2V4ZM8 0V1H12V0H8ZM16 0V1H20V0H16ZM24 0V1H28V0H24ZM32
    0V1H34C34.1374 1 34.266 1.02717 34.3825 1.07549L34.7656 0.151761C34.5298
    0.0539723 34.2712 0 34 0H32ZM36 8H35V12H36V8ZM36 16H35V20H36V16ZM36
    24H35V28H36V24ZM36 32H35V34C35 34.1374 34.9728 34.266 34.9245
    34.3825L35.8482 34.7656C35.946 34.5298 36 34.2712 36 34V32ZM28
    36V35H24V36H28ZM20 36V35H16V36H20ZM12 36V35H8V36H12Z" />
  <g transform="translate(6,6)">
    <Icon.Plus />
  </g>
</svg>
