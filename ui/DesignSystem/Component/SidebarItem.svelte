<script lang="typescript">
  export let active: boolean = false;
  export let indicator: boolean = false;
  export let dataCy: string = "";
  export let onClick: () => void;
</script>

<style>
  .item {
    width: var(--sidebar-width);
    height: 32px;
    margin-bottom: 16px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
  }

  .indicator:hover:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 32px;
    background-color: var(--color-foreground-level-5);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator.active:before {
    position: absolute;
    content: "";
    width: 4px;
    height: 32px;
    background-color: var(--color-primary);
    top: 0px;
    left: 0px;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
  }

  .indicator :global(div:hover svg) {
    fill: var(--color-primary);
  }

  .indicator.active :global(svg) {
    fill: var(--color-primary);
  }
</style>

<div
  class="item indicator"
  class:active
  class:indicator
  data-cy={dataCy}
  on:click|stopPropagation={() => onClick()}>
  <slot />
</div>
