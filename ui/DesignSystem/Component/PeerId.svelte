<script lang="ts">
  import StyledCopyable from "./StyledCopyable.svelte";
  import { Icon } from "../Primitive";

  export let style = "";
  export let dataCy = "";
  export let peerId = "";
  export let truncate: boolean = false;
  export let expandable: boolean = true;
</script>

<style>
  .peer-id {
    display: flex;
  }
</style>

<div class="peer-id" {style} data-cy={dataCy} title={peerId}>
  <Icon.Computer style="margin-right: 0.25rem;" />
  <StyledCopyable {truncate} {expandable} value={peerId} />
</div>
