<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let active: boolean = false;
  export let indicator: boolean = false;
  export let dataCy: string | undefined = undefined;
  export let ariaLabel: string | undefined = undefined;
  export let onClick: (() => void) | undefined = undefined;

  $: clickable = onClick !== undefined;
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
  }

  .clickable {
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

  .indicator.active :global(svg) {
    fill: var(--color-primary);
  }
</style>

<div
  class="item indicator"
  role="button"
  aria-label={ariaLabel}
  class:active
  class:indicator
  class:clickable
  data-cy={dataCy}
  on:click|stopPropagation={() => onClick && onClick()}>
  <div class="button-transition-big" style="display: flex;">
    <slot />
  </div>
</div>
