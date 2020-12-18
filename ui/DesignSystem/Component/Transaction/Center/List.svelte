<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import * as transaction from "../../../../src/transaction";

  import Item from "./Item.svelte";

  export let transactions: transaction.Tx[] = [];

  const dispatch = createEventDispatcher();
  const select = (id: string) => dispatch("select", id);
</script>

<style>
  .list {
    background-color: var(--color-foreground-level-1);
    border-radius: 7px 7px 0 0;
  }
</style>

<div class="list">
  {#each transactions as tx}
    <Item on:click={() => select(tx.hash)} {tx} />
  {/each}
</div>
