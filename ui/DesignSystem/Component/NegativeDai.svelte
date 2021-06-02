<script lang="typescript">
  import Dai from "./Dai.svelte";

  export let style = "";
  export let size = "p"; // h1
</script>

<style>
  .negative-dai {
    display: flex;
    align-items: center;

    padding: 3px 6px;
    border-radius: 0.5rem;

    background-color: var(--color-negative-level-1);
    color: var(--color-negative);
  }

  .negative-dai .symbol:before {
    content: "-";
    color: var(--color-negative);
    margin-left: -1px;
    margin-right: 1px;
  }
</style>

<div class="negative-dai" {style}>
  <Dai {size} color="var(--color-negative)">
    <div class="symbol" />
    <slot />
  </Dai>
</div>
