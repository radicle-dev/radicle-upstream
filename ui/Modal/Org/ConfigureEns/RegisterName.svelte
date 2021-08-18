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
  import { debounce } from "lodash";
  import * as ethers from "ethers";

  import { unreachable } from "ui/src/unreachable";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import * as svelteStore from "ui/src/svelteStore";
  import * as transaction from "ui/src/transaction";
  import * as validation from "ui/src/validation";
  import * as wallet from "ui/src/wallet";

  import { Modal, TextInput } from "ui/DesignSystem";

  import ButtonRow from "./ButtonRow.svelte";
  import ConfirmRegistration from "./ConfirmRegistration.svelte";

  export let registrationDone: (result: Result) => void;
  export let currentName: string | undefined;
  export let fee: ethers.BigNumber;

  type State =
    | {
        type: "validateAndCommit";
      }
    | {
        type: "register";
        commitmentSalt: Uint8Array;
        commitmentBlock: number;
        minimumCommitmentAge: number;
      };

  let state: State = { type: "validateAndCommit" };
  let nameInputValue: string = currentName || "";

  let userInputStarted: boolean = nameInputValue !== "";
  let commitInProgress: boolean = false;

  let validatedName: string;
  let validationState: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };
  let registration: ensResolver.Registration | null;

  const validateFormExecutor = mutexExecutor.create();
  const debouncedValidateFormAndSetState = debounce(
    validateFormAndSetState,
    1000 // 1 second
  );

  $: runAllValidations(nameInputValue);

  function runAllValidations(name: string | undefined): void {
    if (!userInputStarted) {
      userInputStarted = true;
      return;
    }

    if (name) {
      validationState = {
        status: validation.ValidationStatus.Loading,
      };
      debouncedValidateFormAndSetState(name);
    } else {
      debouncedValidateFormAndSetState.cancel();
      validationState = {
        status: validation.ValidationStatus.Error,
        message: "You need to enter a name.",
      };
    }
  }

  async function validateFormAndSetState(name: string): Promise<void> {
    const validationResult = await validateFormExecutor.run(async () => {
      return await runAsyncValidations(name);
    });

    if (validationResult) {
      ({ validatedName, validationState, registration } = validationResult);
    }
  }

  async function runAsyncValidations(name: string): Promise<{
    validatedName: string;
    validationState: validation.ValidationState;
    registration: ensResolver.Registration | null;
  }> {
    const available = await ensRegistrar.isAvailable(name);

    if (available) {
      const accountBalancesStore = wallet.accountBalancesStore;
      const radBalance = svelteStore.get(accountBalancesStore).rad;

      if (radBalance && radBalance < fee) {
        return {
          validatedName: name,
          validationState: {
            status: validation.ValidationStatus.Error,
            message:
              "You don't have enough RAD in your wallet to register this name.",
          },
          registration: null,
        };
      }

      return {
        validatedName: name,
        validationState: {
          status: validation.ValidationStatus.Success,
        },
        registration: null,
      };
    }

    const registration = await ensResolver.getRegistration(
      `${name}.${ensResolver.DOMAIN}`
    );

    const walletStore = svelteStore.get(wallet.store);

    if (registration && registration.owner === walletStore.getAddress()) {
      return {
        validatedName: name,
        validationState: {
          status: validation.ValidationStatus.Success,
        },
        registration,
      };
    }

    return {
      validatedName: name,
      validationState: {
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      },
      registration: null,
    };
  }

  function commitOrGoToUpdateMetadata(): void {
    if (registration) {
      registrationDone({
        name: validatedName,
        registration,
      });
    } else {
      commit(validatedName);
    }
  }

  async function commit(name: string) {
    commitInProgress = true;

    const signNotification = notification.info({
      message:
        "Waiting for you to sign the commitment permit in your connected wallet",
      showIcon: true,
      persist: true,
    });
    let signature: ethers.Signature;

    const deadline = ensRegistrar.deadline();
    try {
      signature = await ensRegistrar.permitSignature(fee, deadline);
    } catch (err) {
      error.show(
        new error.Error({
          message: err.message,
          source: err,
        })
      );
      commitInProgress = false;
      // Don't advance the flow unless the user confirms the signature.
      return;
    } finally {
      signNotification.remove();
    }

    const commitNotification = notification.info({
      message:
        "Waiting for you to confirm the commitment transaction in your connected wallet",
      showIcon: true,
      persist: true,
    });
    const salt = ethers.utils.randomBytes(32);

    let commitResult: ensRegistrar.CommitResult;
    try {
      commitResult = await ensRegistrar.commit(
        name,
        salt,
        fee,
        signature,
        deadline
      );
    } catch (err) {
      error.show(
        new error.Error({
          message: err.message,
          source: err,
        })
      );
      commitInProgress = false;
      // Don't advance flow unless the user confirms the tx.
      return;
    } finally {
      commitNotification.remove();
    }
    transaction.add(transaction.commitEnsName(commitResult.tx));

    const txNotification = notification.info({
      message: "Waiting for the transaction to be included",
      showIcon: true,
      persist: true,
    });

    let receipt;
    try {
      receipt = await commitResult.tx.wait(1);
    } catch (err) {
      error.show(
        new error.Error({
          message: err.message,
          source: err,
        })
      );
      commitInProgress = false;
      // Don't advance flow unless we have the tx receipt.
      return;
    } finally {
      txNotification.remove();
    }

    state = {
      type: "register",
      minimumCommitmentAge: commitResult.minimumCommitmentAge,
      commitmentBlock: receipt.blockNumber,
      commitmentSalt: salt,
    };
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
        validation.ValidationStatus.Success || commitInProgress} />
  </Modal>
{:else if state.type === "register"}
  <ConfirmRegistration
    name={validatedName}
    commitmentSalt={state.commitmentSalt}
    commitmentBlock={state.commitmentBlock}
    minimumCommitmentAge={state.minimumCommitmentAge}
    {registrationDone} />
{:else}
  {unreachable(state)}
{/if}
