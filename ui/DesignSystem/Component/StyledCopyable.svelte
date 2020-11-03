<script lang="ts">
  import Copyable from "./Copyable.svelte";
  import Hoverable from "./Hoverable.svelte";

  export let style = "";
  export let value: string;
  export let notificationText = "Copied to your clipboard";
  export let truncate: boolean = false;
  export let expandable: boolean = true;

  const [head, tail] = value.split(/(.{8}).*(.{8})/).filter(Boolean);

  let hover = false;
</script>

<style>
  .wrapper {
    display: flex;
    position: relative;
  }
</style>

<Hoverable bind:hovering={hover}>
  <div class="wrapper" {style}>
    <Copyable
      style="align-items: center; color: var(--color-foreground-level-6)"
      copyContent={value}
      {notificationText}
      styleContent={hover}
      showIcon={true}>
      {#if !truncate || (expandable && hover)}
        <p class="typo-text-small-mono">{value}</p>
      {:else}
        <p class="typo-text-small-mono">{head}…{tail}</p>
      {/if}
    </Copyable>
  </div>
</Hoverable>
