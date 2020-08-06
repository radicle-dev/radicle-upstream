<script>
  import Copyable from "./Copyable.svelte";
  import { Icon } from "../Primitive";

  export let urn = null;
  export let showOnHover = false;
  export let notificationTxt = null;

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

  .urn p {
    color: var(--color-foreground-level-6);
  }
</style>

<div
  class="wrapper urn"
  data-cy="urn"
  on:mouseover={showFullUrn}
  on:mouseout={hideFullUrn}>
  <Copyable
    iconSize="small"
    style="align-items: center;"
    copyContent={urn}
    {notificationTxt}>
    {#if urn.length > 24}
      {#if (showOnHover && !hover) || !showOnHover}
        <p class="typo-text-small-mono">{firstSix}</p>
        <Icon.Ellipses size="small" />
        <p class="typo-text-small-mono" style="padding-right: 0.25rem">
          {lastSix}
        </p>
      {:else if showOnHover && hover}
        <p class="typo-text-small-mono" style="padding-right: 0.25rem;">
          {cleanUrn}
        </p>
      {/if}
    {:else}
      <p class="typo-text-small-mono" style="padding-right: 0.25rem">{urn}</p>
    {/if}
  </Copyable>
</div>
