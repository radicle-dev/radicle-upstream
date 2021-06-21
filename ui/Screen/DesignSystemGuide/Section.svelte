<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script>
  export let title = null;
  export let subTitle = null;
</script>

<style>
  .section {
    margin-bottom: 180px;
    display: grid;
    grid-template-columns: 342px auto 100px;
  }

  .titles {
    grid-column-start: 1;
    margin-right: 90px;
  }

  h2 {
    margin-bottom: 8px;
  }

  p {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="section">
  <div class="titles">
    <h2>{title}</h2>
    <p>{subTitle}</p>
  </div>
  <div class="content">
    <slot />
  </div>
</div>
