<script>
  import Copyable from "./Copyable.svelte";
  import { Icon, Text } from "../Primitive";

  export let urn = null;
  export let showOnHover = false;

  const cleanUrn = urn.replace(/^%?rad:git:/, "");

  const firstSix = cleanUrn.substring(0, 7);
  const lastSix = cleanUrn.substring(cleanUrn.length - 7, cleanUrn.length);

  let hover = false;

  const hideFullUrn = () => {
    hover = false;
  };
  const showFullUrn = () => {
    hover = true;
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    position: relative;
  }
  .urn {
    background-color: var(--color-foreground-level-2);
    padding: 0 4px;
    border-radius: 4px;
  }
</style>

<div
  class="wrapper urn"
  data-cy="urn"
  on:mouseover={showFullUrn}
  on:mouseout={hideFullUrn}>
  <Copyable iconSize="small" style="align-items: center;" copyContent={urn}>
    {#if urn.length > 24}
      {#if (showOnHover && !hover) || !showOnHover}
        <Text
          style="font-family: var(--typeface-mono-medium); font-size: 14px;
          color: var(--color-foreground-level-6);">
          {firstSix}
        </Text>
        <Icon.Ellipses size="small" />
        <Text
          style="font-family: var(--typeface-mono-medium); font-size: 14px;
          color: var(--color-foreground-level-6); padding-right: 0.25rem">
          {lastSix}
        </Text>
      {:else if showOnHover && hover}
        <Text
          style="font-family: var(--typeface-mono-medium); font-size: 14px;
          color: var(--color-foreground-level-6); padding-right: 0.25rem;">
          {cleanUrn}
        </Text>
      {/if}
    {:else}
      <Text
        style="font-family: var(--typeface-mono-medium); font-size: 14px; color:
        var(--color-foreground-level-6); padding-right: 0.25rem">
        {urn}
      </Text>
    {/if}
  </Copyable>
</div>
