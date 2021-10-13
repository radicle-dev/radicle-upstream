<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;
</script>

<svg
  data-cy={dataCy}
  on:click
  style={`flex-shrink: 0; ${style ? style : ""}`}
  width="24"
  height="24"
  fill="var(--color-foreground-level-5)"
  viewBox={`0 0 24 24`}
  xmlns="http://www.w3.org/2000/svg">
  <slot />
</svg>
