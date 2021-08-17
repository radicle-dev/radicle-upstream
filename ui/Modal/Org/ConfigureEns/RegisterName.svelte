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
  import * as lodash from "lodash";

  import { unreachable } from "ui/src/unreachable";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as svelteStore from "ui/src/svelteStore";
  import * as validation from "ui/src/validation";
  import * as wallet from "ui/src/wallet";

  import { Modal, TextInput } from "ui/DesignSystem";

  import ButtonRow from "./ButtonRow.svelte";
  import ConfirmRegistration from "./ConfirmRegistration.svelte";

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

  let userInputStarted: boolean = nameInputValue !== "";
  let validationState: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };
  let registration: ensResolver.Registration | null;

  $: validateName(nameInputValue);

  function validateName(nameInputValue: string | undefined): void {
    if (!userInputStarted) {
      userInputStarted = true;
      return;
    }

    if (!nameInputValue) {
      validationState = {
        status: validation.ValidationStatus.Error,
        message: "You need to enter a name.",
      };
    } else {
      validationState = {
        status: validation.ValidationStatus.Loading,
      };
      debouncedValidateFormAndSetState();
    }
  }

  const debouncedValidateFormAndSetState = lodash.debounce(
    validateFormAndSetState,
    1000
  );

  const validateFormExecutor = mutexExecutor.create();

  async function validateFormAndSetState(): Promise<void> {
    const validationResult = await validateFormExecutor.run(async () => {
      return await validateForm();
    });

    if (validationResult) {
      ({ validationState, registration } = validationResult);
    }
  }

  async function validateForm(): Promise<{
    validationState: validation.ValidationState;
    registration: ensResolver.Registration | null;
  }> {
    const available = await ensRegistrar.isAvailable(nameInputValue);

    if (available) {
      const accountBalancesStore = wallet.accountBalancesStore;
      const radBalance = svelteStore.get(accountBalancesStore).rad;

      if (radBalance && radBalance < fee) {
        return {
          validationState: {
            status: validation.ValidationStatus.Error,
            message:
              "You don't have enough RAD in your wallet to register this name.",
          },
          registration: null,
        };
      }

      return {
        validationState: {
          status: validation.ValidationStatus.Success,
        },
        registration: null,
      };
    }

    registration = await ensResolver.getRegistration(
      `${nameInputValue}.${ensResolver.DOMAIN}`
    );

    const walletStore = svelteStore.get(wallet.store);

    if (registration && registration.owner === walletStore.getAddress()) {
      return {
        validationState: {
          status: validation.ValidationStatus.Success,
        },
        registration,
      };
    }

    return {
      validationState: {
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      },
      registration: null,
    };
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
      validation={validationState}
      suffix={`.${ensResolver.DOMAIN}`}
      placeholder="Your organization name"
      style="margin: 16px auto; width: 352px;" />

    <ButtonRow
      onSubmit={commitOrGoToUpdateMetadata}
      confirmCopy="Continue"
      disableButtons={validationState.status !==
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
