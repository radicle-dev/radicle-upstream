<script>
  import Copyable from "./Copyable.svelte";
  import { Text } from "../Primitive";

  export let urn = null;
</script>

<style>
  .wrapper {
    background-color: var(--color-foreground-level-1);
    padding: 0 4px;
    border-radius: 4px;
  }
</style>

<div class="wrapper" data-cy="urn">
  <Copyable iconSize="small" style="overflow: hidden; align-items: center; ">
    <Text
      style="font-family: var(--typeface-mono-medium); font-size: 14px; color:
      var(--color-foreground-level-5); overflow: hidden;text-overflow:
      ellipsis;white-space: nowrap;">
      {urn}
    </Text>
  </Copyable>
</div>
