<script lang="typescript">
  export let style = "";
</script>

<style>
  .gwei {
    display: flex;
    align-items: center;

    padding: 3px 8px;
    border-radius: 3px;

    color: var(--color-negative);
    background-color: var(--color-negative-level-1);
  }

  .symbol {
    margin-right: 5px;
  }

  .unit {
    margin-left: 5px;
  }
</style>

<div class="gwei typo-semi-bold cost" {style}>
  <div class="symbol">-</div>
  <slot />
  <div class="unit">gwei</div>
</div>
