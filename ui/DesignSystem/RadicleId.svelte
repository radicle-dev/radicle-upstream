<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import StyledCopyable from "./StyledCopyable.svelte";
  import Icon from "./Icon";

  export let style = "";
  export let urn = "";
  export let truncate: boolean = false;
  export let showIcon: boolean = true;

  const hash = urn.replace("rad:git:", "");
</script>

<style>
  .radicle-id {
    display: flex;
  }
</style>

<div class="radicle-id" {style} data-cy="urn" title={urn}>
  <Icon.At style="margin-right: 0.25rem;" />
  <StyledCopyable {truncate} value={hash} copyContent={urn} {showIcon} />
</div>
