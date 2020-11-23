<script lang="ts">
  import StyledCopyable from "./StyledCopyable.svelte";
  import { Icon } from "../Primitive";

  export let style = "";
  export let urn = "";
  export let truncate: boolean = false;

  const hash = urn.replace("rad:git:", "");
</script>

<style>
  .peer-id {
    display: flex;
  }
</style>

<div class="radicle-id" {style} data-cy="urn" title={urn}>
  <Icon.At style="margin-right: 0.25rem;" />
  <StyledCopyable {truncate} value={hash} copyContent={urn} />
</div>
