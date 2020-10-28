<script lang="ts">
  import { Icon } from "../Primitive";
  import Copyable from "./Copyable.svelte";

  export let style: string = "";
  export let urn: string = "";

  const hash = urn.replace("rad:git:", "");
</script>

<style>
  .shareable-identifier {
    cursor: pointer;
    color: var(--color-foreground-level-6);
    user-select: none;
    display: flex;
  }
</style>

<div data-cy="urn" class="shareable-identifier typo-text" {style}>
  <Copyable
    showIcon={false}
    styleContent={false}
    copyContent={urn}
    notificationText="Handle copied to clipboard">
    <Icon.At style="margin-right: 0.5rem;" />
    <p class="typo-overflow-ellipsis">{hash}</p>
  </Copyable>
</div>
