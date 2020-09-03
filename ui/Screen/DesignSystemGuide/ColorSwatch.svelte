<script>
  import { Tooltip } from "../../DesignSystem/Component";
  import CopyToClipboard from "./CopyToClipboard.svelte";

  export let style = null;
  export let color = null;
</script>

<style>
  .wrapper {
    display: inline-flex;
  }
  .color {
    width: 4rem;
    height: 4rem;
    border-radius: 0.25rem;
  }
  .border {
    border: 1px solid var(--color-foreground-level-3);
  }
</style>

<div class="wrapper" {style}>
  <CopyToClipboard
    value={`var(${color});`}
    notificationText="The color was copied to your clipboard">
    <Tooltip value={`var(${color})`} position="top">
      <div
        class:border={color === '--color-background'}
        class="color"
        style={`background-color: var(${color});`} />
    </Tooltip>
  </CopyToClipboard>
</div>
