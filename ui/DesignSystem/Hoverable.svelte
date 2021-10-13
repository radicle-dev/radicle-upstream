<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  export let hovering: boolean = false;
  export let style: string | undefined = undefined;

  const enter = () => (hovering = true);
  const leave = () => (hovering = false);
</script>

<div {style} on:mouseenter={enter} on:mouseleave={leave}>
  <slot {hovering} />
</div>
