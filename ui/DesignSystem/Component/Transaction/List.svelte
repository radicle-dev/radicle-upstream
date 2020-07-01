<script>
  import { createEventDispatcher } from "svelte";

  import Item from "./Item.svelte";

  export let transactions = null;

  const dispatch = createEventDispatcher();
  const select = id => dispatch("select", id);
</script>

<style>
  .list {
    background-color: var(--color-foreground-level-1);
    border-radius: 7px 7px 0 0;
  }
</style>

<div class="list">
  {#each transactions as tx (tx.id)}
    <Item on:click={() => select(tx.id)} {tx} />
  {/each}
</div>
