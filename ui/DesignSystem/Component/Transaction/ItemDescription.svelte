<script>
  import { Text } from "../../Primitive";

  export let message = null;
  export let state = null;

  const colors = {
    confirmed: "var(--color-caution)",
    failed: "var(--color-negative)",
    pending: "var(--color-caution)",
    settled: "var(--color-positive)",
  };
  $: color = colors[state];
</script>

<style>
  .description {
    align-self: center;
    display: flex;
    flex-direction: column;
  }
</style>

<div class="description">
  <Text variant="small" style="width: max-content">{message}</Text>
  <Text variant="small" style="color: {color}; text-transform: capitalize;">
    {state}
  </Text>
</div>
