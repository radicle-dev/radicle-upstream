<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Icon from "./Icon";

  export let style = "";
  export let color = "var(--color-foreground-level-6)";
  export let size = "p"; // h1

  function getSize(): number {
    switch (size) {
      case "h1":
        return 36;
      default:
        return 20;
    }
  }
</script>

<style>
  .dai {
    display: flex;
    align-items: center;

    color: var(--color-foreground-level-6);
  }
</style>

<div class="dai" style="{style}; color: {color};">
  <Icon.CurrencyDAI
    style="fill: {color}; padding-top: 3px; width: {getSize()}px; height: {getSize()}px;" />
  <slot />
</div>
