<script>
  // TODO(sarah): write tests for this once it's implemented in the ui somewhere
  import { copyToClipboard } from "../../lib/nativeUtils.js";

  import { Icon, Text } from "../Primitive";

  let slotContent;

  const copy = () => copyToClipboard(slotContent.textContent);
</script>

<style>
  .container {
    display: flex;
    cursor: pointer;
    color: var(--color-gray);
  }
</style>

<div class="container" on:click={copy}>
  <Text>
    <span bind:this={slotContent}>
      <slot />
    </span>
  </Text>
  <svelte:component this={Icon.Copy} style="margin-left: 8px;" />
</div>
