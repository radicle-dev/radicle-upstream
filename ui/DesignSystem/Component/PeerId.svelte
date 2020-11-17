<script lang="ts">
  import StyledCopyable from "./StyledCopyable.svelte";
  import { Icon } from "../Primitive";

  export let style = "";
  export let dataCy = "";
  export let peerId = "";
</script>

<style>
  .peer-id {
    display: flex;
  }
</style>

<div class="peer-id" {style} {dataCy}>
  <Icon.Computer style="margin-right: 0.25rem;" />
  <StyledCopyable value={peerId} />
</div>
