<script lang="ts">
  import StyledCopyable from "./StyledCopyable.svelte";
  import { Icon } from "./";

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
