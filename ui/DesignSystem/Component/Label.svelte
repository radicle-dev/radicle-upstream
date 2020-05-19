<script>
  import { Text } from "../Primitive";
  export let title = null;
  export let color = null;
  export let style = null;
</script>

<style>
  .label {
    background-color: var(--color);
    padding: 2px 6px;
    height: 24px;
    border-radius: 2px;
  }
</style>

<div class="label" style="--color: {color}; {style}">
  <Text variant="small">{title}</Text>
</div>
