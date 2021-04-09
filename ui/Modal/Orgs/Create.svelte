<script>
  import { Modal } from "../../DesignSystem/Component";
  import { Button } from "../../DesignSystem/Primitive";
  import { store, Status } from "../../src/wallet.ts";
  import * as org from "../../src/org.ts";

  const orgStore = org.store;

  function createOrg(owner) {
    org.createOrg(owner, $store.signer);
  }

  $: wallet = $store;
</script>

<style>
</style>

{#if $wallet.status === Status.Connected}
  <Modal>
    <p>Create Org</p>
    <p>Owner: {$wallet.connected.account.address}</p>
    <p>Status: {$orgStore}</p>
    <Button on:click={() => createOrg($wallet.connected.account.address)}>
      Create
    </Button>
  </Modal>
{:else}
  <p>Not connected</p>
{/if}
