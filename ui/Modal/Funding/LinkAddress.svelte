<script lang="typescript">
  import EthToRadicle from "./Link/EthToRadicle.svelte";
  import SavedToRadicle from "./Link/SavedToRadicle.svelte";
  import RadicleToEth from "./Link/RadicleToEth.svelte";
  import { Remote } from "../../DesignSystem/Component";

  import {
    ClaimsContract,
    claimsAddress,
  } from "../../src/attestation/contract";
  import * as identity from "../../src/identity";
  import { store as walletStore } from "../../src/wallet";
  import { session } from "../../src/session";

  import * as modal from "../../src/modal";

  function onCancel(): void {
    modal.hide();
  }

  enum Step {
    EthToRadicle = "EthToRadicle",
    SavedToRadicle = "SavedToRadicle",
    RadicleToEth = "RadicleToEth",
  }

  let currentStep = Step.EthToRadicle;

  function onContinue() {
    switch (currentStep) {
      case Step.EthToRadicle:
        currentStep = Step.SavedToRadicle;
        break;
      case Step.SavedToRadicle:
        currentStep = Step.RadicleToEth;
        break;
      case Step.RadicleToEth:
        modal.hide();
        break;
    }
  }

  $: address = $walletStore.account()?.address || "";

  async function claimEthAddress() {
    await identity.claimEthAddress(address);
    onContinue();
  }

  async function claimRadicleIdentity(identity: identity.Identity) {
    const claims = new ClaimsContract(
      $walletStore.signer,
      claimsAddress($walletStore.environment)
    );
    await claims.claim(identity.urn);
    onContinue();
  }

</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;

    text-align: center;
  }

</style>

<Remote store={session} let:data={it}>
  <div class="wrapper">
    {#if currentStep === Step.EthToRadicle}
      <EthToRadicle
        {address}
        identity={it.identity}
        {onCancel}
        onConfirm={claimEthAddress} />
    {:else if currentStep === Step.SavedToRadicle}
      <SavedToRadicle {onCancel} {onContinue} />
    {:else if currentStep === Step.RadicleToEth}
      <RadicleToEth
        {address}
        identity={it.identity}
        {onCancel}
        onSendTransaction={() => claimRadicleIdentity(it.identity)} />
    {/if}
  </div>
</Remote>
