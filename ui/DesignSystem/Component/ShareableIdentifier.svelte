<script lang="ts">
  import { Icon } from "../Primitive";
  import Copyable from "./Copyable.svelte";

  export let urn = "";
  const hash = urn.replace("rad:git:", "");
</script>

<style>
  .shareable-identifier {
    cursor: pointer;
    color: var(--color-foreground-level-6);
  }
</style>

<div class="shareable-identifier typo-text">
  <Copyable showIcon={false} styleContent={false} copyContent={urn}>
    <Icon.At style="margin-right: 0.5rem;" />
    {hash}
  </Copyable>
</div>
