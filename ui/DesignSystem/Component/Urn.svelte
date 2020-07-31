<script>
  import Copyable from "./Copyable.svelte";
  import { Icon, Text } from "../Primitive";

  export let urn = null;
  const firstSix = urn.substring(0, 7);
  const lastSix = urn.substring(urn.length - 7, urn.length);
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    background-color: var(--color-foreground-level-1);
    padding: 0 4px;
    border-radius: 4px;
  }
</style>

<div class="wrapper" data-cy="urn">
  <Copyable iconSize="small" style="align-items: center;" copyContent={urn}>
    <Text
      style="font-family: var(--typeface-mono-medium); font-size: 14px; color:
      var(--color-foreground-level-5);">
      {firstSix}
    </Text>
    <Icon.Ellipses size="small" />
    <Text
      style="font-family: var(--typeface-mono-medium); font-size: 14px; color:
      var(--color-foreground-level-5); padding-right: 0.25rem">
      {lastSix}
    </Text>
  </Copyable>
</div>
