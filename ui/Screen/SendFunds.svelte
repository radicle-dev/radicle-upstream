<script>
  import { pop } from "svelte-spa-router";

  import { fromStore, toStore, amountStore } from "../src/transfer.ts";

  let fromAddress;
  fromStore.subscribe(value => {
    fromAddress = value;
  });
  let toAddress;
  toStore.subscribe(value => {
    toAddress = value;
  });
  let amount;
  amountStore.subscribe(value => {
    amount = value;
  });
  console.log(fromAddress, " - ", toAddress, " - ", amount);
</script>

<p>hello</p>
<button on:click={() => pop()}>pop</button>
