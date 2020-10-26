<script lang="ts">
  import Copyable from "./Copyable.svelte";
  import Hoverable from "./Hoverable.svelte";
  import { Icon } from "../Primitive";

  export let value: string;
  export let notificationText = "Copied to your clipboard";
  export let truncate: boolean = false;

  const firstSix = value.substring(0, 7);
  const lastSix = value.substring(value.length - 7, value.length);

  let hover = false;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: center;
    position: relative;
  }
</style>

<Hoverable bind:hovering={hover}>
  <div class="wrapper">
    <Copyable
      style="align-items: center; color: var(--color-foreground-level-6)"
      copyContent={value}
      {notificationText}
      styleContent={hover}
      showIcon={true}>
      {#if !truncate || hover}
        <p class="typo-text-small-mono">{value}</p>
      {:else}
        <p class="typo-text-small-mono">{firstSix}</p>
        <Icon.EllipsisSmall />
        <p class="typo-text-small-mono">{lastSix}</p>
      {/if}
    </Copyable>
  </div>
</Hoverable>
