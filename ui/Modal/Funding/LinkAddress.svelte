<script lang="typescript">
  import EthToRadicle from "./Link/EthToRadicle.svelte";
  import EnterPassphrase from "./Link/EnterPassphrase.svelte";
  import SavedToRadicle from "./Link/SavedToRadicle.svelte";
  import RadicleToEth from "./Link/RadicleToEth.svelte";
  import { Remote } from "../../DesignSystem/Component";

  import * as identity from "../../src/identity";
  import { store as walletStore } from "../../src/wallet";
  import { session } from "../../src/session";

  import * as modal from "../../src/modal";

  function onCancel(): void {
    modal.hide();
  }

  enum Step {
    RadicleToEth = "RadicleToEth",
    EthToRadicle = "EthToRadicle",
    EnterPassphrase = "EnterPassphrase",
    SavedToRadicle = "SavedToRadicle",
  }

  let currentStep = Step.RadicleToEth;

  function onContinue() {
    switch (currentStep) {
      case Step.RadicleToEth:
        currentStep = Step.EthToRadicle;
        break;
      case Step.EthToRadicle:
        currentStep = Step.EnterPassphrase;
        break;
      case Step.EnterPassphrase:
        identity.ethereumAddress.set(address);
        currentStep = Step.SavedToRadicle;
        break;
      case Step.SavedToRadicle:
        modal.hide();
        break;
    }
  }

  async function claimRadicleIdentity() {
    // TODO(nuno): call actual contract
    await new Promise(res => setTimeout(res, 2500));
    onContinue();
  }

  // Values
  let passphrase: string = "";

  $: address = $walletStore.account()?.address || "";
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
    {#if currentStep === Step.RadicleToEth}
      <RadicleToEth
        {address}
        identity={it.identity}
        {onCancel}
        onSendTransaction={claimRadicleIdentity} />
    {:else if currentStep === Step.EthToRadicle}
      <EthToRadicle
        {address}
        identity={it.identity}
        {onCancel}
        onConfirm={onContinue} />
    {:else if currentStep === Step.EnterPassphrase}
      <EnterPassphrase bind:passphrase {onCancel} onConfirm={onContinue} />
    {:else if currentStep === Step.SavedToRadicle}
      <SavedToRadicle {onCancel} {onContinue} />
    {/if}
  </div>
</Remote>
