<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ethers from "ethers";

  import { unreachable } from "ui/src/unreachable";
  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import { Modal, TextInput } from "ui/DesignSystem";

  import ConfirmRegistration from "./ConfirmRegistration.svelte";
  import ButtonRow from "./ButtonRow.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as validation from "ui/src/validation";

  export let done: () => void;
  export let name: string = "";
  export let fee: ethers.BigNumber;

  type State =
    | {
        type: "commit";
      }
    | {
        type: "register";
        commitmentSalt: Uint8Array;
        commitmentBlock: number;
        minAge: number;
      };

  let state: State = { type: "commit" };

  let validationStatus: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

  let timeoutHandle: number;
  let userInputStarted: boolean = name !== "";
  let nextStep: "registerName" | "updateMetadata";

  function validateName(name: string | undefined): void {
    if (!userInputStarted) {
      userInputStarted = true;
      return;
    }

    if (!name) {
      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "You need to enter a name.",
      };
    } else {
      validationStatus = {
        status: validation.ValidationStatus.Loading,
      };
      if (timeoutHandle) {
        clearTimeout(timeoutHandle);
      }
      timeoutHandle = window.setTimeout(() => {
        checkNameAvailability();
      }, 1000);
    }
  }

  async function checkNameAvailability(): Promise<void> {
    const available = await ensRegistrar.isAvailable(name);

    if (available) {
      const accountBalancesStore = wallet.accountBalancesStore;
      const radBalance = svelteStore.get(accountBalancesStore).rad;

      if (radBalance && radBalance < fee) {
        validationStatus = {
          status: validation.ValidationStatus.Error,
          message:
            "You don't have enough RAD in your wallet to register this name.",
        };

        return;
      }

      validationStatus = { status: validation.ValidationStatus.Success };
      nextStep = "registerName";
    } else {
      const registration = await ensResolver.getRegistration(
        `${name}.${ensResolver.DOMAIN}`
      );

      const walletStore = svelteStore.get(wallet.store);

      if (registration && registration.owner === walletStore.getAddress()) {
        validationStatus = { status: validation.ValidationStatus.Success };
        nextStep = "updateMetadata";
        return;
      }

      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      };
    }
  }

  async function handleSubmit(): Promise<void> {
    if (nextStep === "registerName") {
      commit();
    } else {
      done();
    }
  }

  async function commit() {
    // TODO: show notification to confirm tx in wallet

    try {
      const salt = ethers.utils.randomBytes(32);

      const walletStore = svelteStore.get(wallet.store);

      const commitResult = await ensRegistrar.commit(
        walletStore.environment,
        name,
        salt,
        fee
      );

      state = {
        type: "register",
        minAge: commitResult.minAge,
        commitmentBlock: commitResult.receipt.blockNumber,
        commitmentSalt: salt,
      };
    } catch (err) {
      error.show(
        new error.Error({
          message:
            "Transaction failed. Please try again and confirm the signature & transaction in your connected wallet.",
          source: err,
        })
      );
    }
  }

  $: validateName(name);
</script>

{#if state.type === "commit"}
  <Modal
    emoji="📇"
    title="Let’s name your org"
    desc="What should your org be called? This name will show up on the top of your profile and anywhere you interact as an org on Radicle.">
    <TextInput
      bind:value={name}
      showSuccessCheck
      validation={validationStatus}
      suffix={`.${ensResolver.DOMAIN}`}
      placeholder="Your organization name"
      style="margin: 16px auto; width: 352px;" />

    <ButtonRow
      onSubmit={handleSubmit}
      confirmCopy="Continue"
      disableButtons={validationStatus.status !==
        validation.ValidationStatus.Success} />
  </Modal>
{:else if state.type === "register"}
  <ConfirmRegistration
    {name}
    commitmentSalt={state.commitmentSalt}
    commitmentBlock={state.commitmentBlock}
    minAge={state.minAge}
    {done} />
{:else}
  {unreachable(state)}
{/if}
