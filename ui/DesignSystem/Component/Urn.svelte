<script>
  import Copyable from "./Copyable.svelte";
  import { Icon, Text } from "../Primitive";

  export let urn = null;
  export let showOnHover = false;

  const cleanUrn = urn.includes("rad:git:")
    ? urn.substring(8, urn.length)
    : urn;
  console.log(cleanUrn);

  const firstSix = cleanUrn.substring(0, 7);
  const lastSix = cleanUrn.substring(cleanUrn.length - 7, cleanUrn.length);

  const fullUrn = { className: "hidden" };

  const hideFullUrn = () => {
    fullUrn.className = "hidden";
  };
  const showFullUrn = () => {
    fullUrn.className = "visible";
  };
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    position: relative;
  }
  .urn {
    background-color: var(--color-foreground-level-1);
    padding: 0 4px;
    border-radius: 4px;
  }
  .fullUrn {
    position: absolute;
    top: 0;
    left: 0;
  }
  .fullUrn.visible {
    visibility: visible;
  }
  .fullUrn.hidden {
    visibility: hidden;
  }
</style>

<div
  class="wrapper urn"
  data-cy="urn"
  on:mouseover={showFullUrn}
  on:mouseout={hideFullUrn}>
  {#if showOnHover}
    <div class={`fullUrn urn ${fullUrn.className}`}>
      <Copyable iconSize="small" style="align-items: center;" copyContent={urn}>
        <Text
          style="font-family: var(--typeface-mono-medium); font-size: 14px;
          color: var(--color-foreground-level-5); padding-right: 0.25rem;">
          {cleanUrn}
        </Text>
      </Copyable>
    </div>
  {/if}
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
