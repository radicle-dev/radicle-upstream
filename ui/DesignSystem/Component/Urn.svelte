<script>
  import Copyable from "./Copyable.svelte";
  import { Icon } from "../Primitive";

  export let urn = null;
  export let showOnHover = false;
  export let notificationText = "Copied to your clipboard";

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
</style>

<div
  class="wrapper urn"
  data-cy="urn"
  on:mouseover={showFullUrn}
  on:mouseout={hideFullUrn}>
  <Copyable style="align-items: center;" copyContent={urn} {notificationText}>
    {#if urn.length > 24}
      {#if (showOnHover && !hover) || !showOnHover}
        <p class="typo-text-small-mono">{firstSix}</p>
        <Icon.EllipsesSmall />
        <p class="typo-text-small-mono">{lastSix}</p>
      {:else if showOnHover && hover}
        <p data-cy="full-urn" class="typo-text-small-mono">{cleanUrn}</p>
      {/if}
    {:else}
      <p class="typo-text-small-mono">{urn}</p>
    {/if}
  </Copyable>
</div>
