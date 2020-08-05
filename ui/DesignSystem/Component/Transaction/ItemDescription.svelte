<script>
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
  <p class="typo-text-small" style="width: max-content">{message}</p>
  <p
    class="typo-text-small"
    style="color: {color}; text-transform: capitalize;">
    {state}
  </p>
</div>
