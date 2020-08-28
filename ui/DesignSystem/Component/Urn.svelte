<script>
  import Copyable from "./Copyable.svelte";
  import Hoverable from "./Hoverable.svelte";
  import { Icon } from "../Primitive";

  export let notificationText = "Copied to your clipboard";
  export let showOnHover = false;
  export let truncate = false;
  export let urn = null;

  const cleanUrn = urn.replace(/^%?rad:git:/, "");

  const firstSix = cleanUrn.substring(0, 7);
  const lastSix = cleanUrn.substring(cleanUrn.length - 7, cleanUrn.length);

  let hover;

  $: expanded = truncate ? showOnHover && hover : true;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    position: relative;
  }
</style>

<Hoverable bind:hovering={hover}>
  <div class="wrapper urn" data-cy="urn">
    <Copyable style="align-items: center;" copyContent={urn} {notificationText}>
      {#if urn.length > 24}
        {#if expanded}
          <p data-cy="full-urn" class="typo-text-small-mono">{cleanUrn}</p>
        {:else}
          <p class="typo-text-small-mono">{firstSix}</p>
          <Icon.EllipsisSmall />
          <p class="typo-text-small-mono">{lastSix}</p>
        {/if}
      {:else}
        <p class="typo-text-small-mono">{urn}</p>
      {/if}
    </Copyable>
  </div>
</Hoverable>
