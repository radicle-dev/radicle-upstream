<script lang="typescript">
  import * as svelteStore from "svelte/store";
  import * as ethers from "ethers";

  import EthToRadicle from "./Link/EthToRadicle.svelte";
  import EnterPassphrase from "./Link/EnterPassphrase.svelte";
  import SavedToRadicle from "./Link/SavedToRadicle.svelte";
  import RadicleToEth from "./Link/RadicleToEth.svelte";
  import { Remote } from "../../DesignSystem/Component";

  import { claims, claimsAddress } from "../../src/funding/contract";
  import type { Identity } from "../../scr/identity";
  import * as identity from "../../src/identity";
  import * as transaction from "../../src/transaction";

  import { store as walletStore } from "../../src/wallet";
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

  $: address = svelteStore.get(walletStore).account()?.address || "";

  function onContinue() {
    switch (currentStep) {
      case Step.EthToRadicle:
        currentStep = Step.EnterPassphrase;
        break;
      case Step.EnterPassphrase:
        identity.ethereumAddress.set(address);
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

  async function claimRadicleIdentity(identity: Identity) {
    const claimsContract = claims(
      $walletStore.signer,
      claimsAddress($walletStore.environment)
    );
    const payload = ethers.utils.toUtf8Bytes(identity.peerId);
    await claimsContract
      .claim(0, payload)
      .then((ctx: ethers.ContractTransaction) => {
        transaction.add(transaction.claimRadicleIdentity(ctx, identity));
        onContinue();
      });
  }

  // Values
  let passphrase: string = "";
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
        onSendTransaction={() => claimRadicleIdentity(it.identity)} />
    {/if}
  </div>
</Remote>
