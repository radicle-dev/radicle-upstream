<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript" context="module">
  export interface Result {
    registration: ensResolver.Registration | null;
    name: string;
  }
</script>

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

  export let registrationDone: (result: Result) => void;
  export let currentName: string | undefined;
  export let fee: ethers.BigNumber;

  let nameInputValue: string = currentName || "";

  type State =
    | {
        type: "validateAndCommit";
      }
    | {
        type: "register";
        commitmentSalt: Uint8Array;
        commitmentBlock: number;
        minAge: number;
      };

  let state: State = { type: "validateAndCommit" };

  let validationStatus: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

  let timeoutHandle: number;
  let userInputStarted: boolean = nameInputValue !== "";
  let registration: ensResolver.Registration | null;

  $: validateName(nameInputValue);

  function validateName(nameInputValue: string | undefined): void {
    if (!userInputStarted) {
      userInputStarted = true;
      return;
    }

    if (!nameInputValue) {
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
        validateFormAndQueueNextAction();
      }, 1000);
    }
  }

  async function validateFormAndQueueNextAction(): Promise<void> {
    const available = await ensRegistrar.isAvailable(nameInputValue);

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
      // Commit to name after user clicks button.
    } else {
      registration = await ensResolver.getRegistration(
        `${nameInputValue}.${ensResolver.DOMAIN}`
      );

      const walletStore = svelteStore.get(wallet.store);

      if (registration && registration.owner === walletStore.getAddress()) {
        validationStatus = { status: validation.ValidationStatus.Success };
        // Go to update metadata after user clicks button.
        return;
      }

      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      };
    }
  }

  async function commitOrGoToUpdateMetadata(): Promise<void> {
    if (registration) {
      registrationDone({
        name: nameInputValue,
        registration,
      });
    } else {
      commit();
    }
  }

  async function commit() {
    // TODO: show notification to confirm tx in wallet

    try {
      const salt = ethers.utils.randomBytes(32);

      const walletStore = svelteStore.get(wallet.store);

      const commitResult = await ensRegistrar.commit(
        walletStore.environment,
        nameInputValue,
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
</script>

{#if state.type === "validateAndCommit"}
  <Modal
    emoji="📇"
    title="Let’s name your org"
    desc="What should your org be called? This name will show up on the top of your profile and anywhere you interact as an org on Radicle.">
    <TextInput
      bind:value={nameInputValue}
      showSuccessCheck
      validation={validationStatus}
      suffix={`.${ensResolver.DOMAIN}`}
      placeholder="Your organization name"
      style="margin: 16px auto; width: 352px;" />

    <ButtonRow
      onSubmit={commitOrGoToUpdateMetadata}
      confirmCopy="Continue"
      disableButtons={validationStatus.status !==
        validation.ValidationStatus.Success} />
  </Modal>
{:else if state.type === "register"}
  <ConfirmRegistration
    name={nameInputValue}
    commitmentSalt={state.commitmentSalt}
    commitmentBlock={state.commitmentBlock}
    minAge={state.minAge}
    {registrationDone} />
{:else}
  {unreachable(state)}
{/if}
