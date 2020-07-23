<script>
  import { pop } from "svelte-spa-router";
  import { get } from "svelte/store";

  import { fromStore, toStore, amountStore } from "../src/transfer.ts";

  const fromAddress = get(fromStore);
  const toAddress = get(toStore);
  const amount = get(amountStore);
  console.log(fromAddress, " - ", toAddress, " - ", amount);
</script>

<p>hello</p>
<button on:click={() => pop()}>pop</button>
