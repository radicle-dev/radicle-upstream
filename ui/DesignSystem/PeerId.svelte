<script lang="typescript">
  import StyledCopyable from "./StyledCopyable.svelte";
  import { Icon } from "./";

  export let style: string = "";
  export let peerId: string = "";
  export let truncate: boolean = false;
  export let expandable: boolean = true;
</script>

<style>
  .peer-id {
    display: flex;
  }
</style>

<div class="peer-id" {style} data-cy="peer-id" title={peerId}>
  <Icon.Computer style="margin-right: 0.25rem;" />
  <StyledCopyable {truncate} {expandable} value={peerId} />
</div>
