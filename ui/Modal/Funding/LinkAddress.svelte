<script lang="typescript">
  import EthToRadicle from "../../DesignSystem/Component/Funding/Link/EthToRadicle.svelte";
  import EnterPassphrase from "../../DesignSystem/Component/Funding/Link/EnterPassphrase.svelte";
  import SavedToRadicle from "../../DesignSystem/Component/Funding/Link/SavedToRadicle.svelte";
  import RadicleToEth from "../../DesignSystem/Component/Funding/Link/RadicleToEth.svelte";
  import { Remote } from "../../DesignSystem/Component";

  import { wallet } from "../../src/wallet";
  import { session } from "../../src/session";

  import * as modal from "../../src/modal";

  function onCancel(): void {
    modal.hide();
  }

  enum Step {
    EthToRadicle = "EthToRadicle",
    EnterPassphrase = "EnterPassphrase",
    SavedToRadicle = "SavedToRadicle",
    RadicleToEth = "RadicleToEth",
  }

  let currentStep = Step.EthToRadicle;

  function onContinue() {
    switch (currentStep) {
      case Step.EthToRadicle:
        currentStep = Step.EnterPassphrase;
        break;
      case Step.EnterPassphrase:
        // TODO(nuno): Add the eth address to the radicle identity
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

  // Values
  let passphrase: string = "";

  $: address = wallet.account()?.address || "";
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 600px;
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
        onConfirm={onContinue} />
    {:else if currentStep === Step.EnterPassphrase}
      <EnterPassphrase bind:passphrase {onCancel} onConfirm={onContinue} />
    {:else if currentStep === Step.SavedToRadicle}
      <SavedToRadicle {onCancel} {onContinue} />
    {:else if currentStep === Step.RadicleToEth}
      <RadicleToEth
        {address}
        identity={it.identity}
        {onCancel}
        onSendTransaction={onContinue} />
    {/if}
  </div>
</Remote>
