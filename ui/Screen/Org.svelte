<script lang="typescript">
  import * as org from "../src/org.ts";
  import { store } from "../src/wallet";

  export let params: { address: string };

  const safeAddr = org.getSafeAddr(params.address, $store.provider);
</script>

<style>
</style>

<div>
  Org page for
  {params.address}
  <p>
    Safe address:
    {#await safeAddr}
      Loading..
    {:then addr}
      {addr}
    {:catch error}
      Error:
      {error}
    {/await}
  </p>
</div>
