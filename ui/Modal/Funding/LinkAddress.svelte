<script lang="typescript">
  import EthToRadicle from "../../DesignSystem/Component/Funding/Link/EthToRadicle.svelte";

  import { wallet } from "../../src/wallet";
  import { session } from "../../src/session";
  import * as identity from "../../src/identity";

  import * as modal from "../../src/modal";

  function onCancel(): void {
    modal.hide();
  }
  async function onConfirm(): Promise<void> {
    console.log("onConfirmed: ", $wallet.connected.account.address);
    const p = identity
      .linkEthereumAddress($wallet.connected.account.address)
      .then(() => modal.hide());
    console.log("onConfirmed, linked: ", identity.linkedAddress);
    return p;
  }
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 600px;
    min-height: 400px;
    background: var(--color-background);
    border-radius: 0.5rem;

    text-align: center;
  }
</style>

<div class="wrapper">
  <EthToRadicle
    address={$wallet.connected.account.address}
    identity={$session.data.identity}
    {onCancel}
    {onConfirm} />
</div>
