<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript" context="module">
  export interface Result {
    registration: ensResolver.Registration | undefined;
    name: string;
  }
</script>

<script lang="typescript">
  import type * as ethers from "ethers";

  import { sleep } from "ui/src/sleep";
  import { unreachable } from "ui/src/unreachable";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import * as svelteStore from "ui/src/svelteStore";
  import * as transaction from "ui/src/transaction";
  import * as validation from "ui/src/validation";
  import * as wallet from "ui/src/wallet";

  import { Button, Modal, TextInput } from "ui/DesignSystem";

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
        commitment: ensRegistrar.Commitment;
        commitmentBlock: number;
      };

  let state: State = { type: "validateAndCommit" };
  let nameInputValue: string = currentName || "";
  let commitInProgress: boolean = false;
  let validatedName: string;
  let validationState: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };
  let registration: ensResolver.Registration | undefined;
  let userInputStarted: boolean = nameInputValue !== "";
  const validateFormExecutor = mutexExecutor.create();

  $: (userInputStarted && validateFormAndSetState(nameInputValue)) ||
    (userInputStarted = true);

  async function validateFormAndSetState(
    name: string | undefined
  ): Promise<void> {
    validationState = {
      status: validation.ValidationStatus.Loading,
    };

    const validationResult = await validateFormExecutor.run(
      async abortSignal => {
        if (!name) {
          return {
            validatedName: "",
            validationState: {
              status: validation.ValidationStatus.NotStarted,
            } as validation.ValidationState,
            registration: undefined,
          };
        }

        // Debouce.
        await sleep(1000);

        if (abortSignal.aborted) {
          return;
        }

        return await runAsyncValidations(name);
      }
    );

    if (validationResult) {
      ({ validatedName, validationState, registration } = validationResult);
    }
  }

  async function runAsyncValidations(name: string): Promise<{
    validatedName: string;
    validationState: validation.ValidationState;
    registration: ensResolver.Registration | undefined;
  }> {
    const available = await ensRegistrar.isAvailable(name);

    if (available) {
      const accountBalancesStore = wallet.accountBalancesStore;
      const radBalance = svelteStore.get(accountBalancesStore).rad;

      if (radBalance && radBalance.lt(fee)) {
        return {
          validatedName: name,
          validationState: {
            status: validation.ValidationStatus.Error,
            message: `You don't have enough RAD in your wallet to register this name. Name registration costs ${ensRegistrar.formatFee(
              fee
            )} RAD.`,
          },
          registration: undefined,
        };
      }

      return {
        validatedName: name,
        validationState: {
          status: validation.ValidationStatus.Success,
        },
        registration: undefined,
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
        message: "Sorry, that name is already taken.",
      },
      registration: undefined,
    };
  }

  async function commitOrGoToUpdateMetadata(): Promise<void> {
    if (registration) {
      registrationDone({
        name: validatedName,
        registration,
      });
    } else {
      try {
        commitInProgress = true;
        state = await commit();
      } catch (err) {
        error.show(
          new error.Error({
            message: err,
            source: err,
          })
        );
      } finally {
        commitInProgress = false;
      }
    }
  }

  async function commit(): Promise<State> {
    const wallet_ = svelteStore.get(wallet.store);
    const walletAddress = wallet_.getAddress()?.toLowerCase();
    const commitment = ensRegistrar.restoreCommitment();

    if (
      commitment &&
      commitment.name === validatedName &&
      commitment.ownerAddress === walletAddress
    ) {
      if (commitment.block) {
        return {
          type: "register",
          commitment,
          commitmentBlock: commitment.block,
        };
      } else {
        const commitNotification = notification.info({
          message: "Waiting for previous commitment transation to be confirmed",
          showIcon: true,
          persist: true,
        });
        let commitmentBlock;
        try {
          const receipt = await wallet_.provider.waitForTransaction(
            commitment.txHash
          );
          commitmentBlock = receipt.blockNumber;
        } finally {
          commitNotification.remove();
        }
        return { type: "register", commitment, commitmentBlock };
      }
    } else {
      return submitCommitment(validatedName);
    }
  }

  async function submitCommitment(name: string): Promise<State> {
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
    } finally {
      signNotification.remove();
    }

    const commitNotification = notification.info({
      message:
        "Waiting for you to confirm the commitment transaction in your connected wallet",
      showIcon: true,
      persist: true,
    });
    const salt = ensRegistrar.generateSalt();

    let commitment: ensRegistrar.Commitment;
    let tx: transaction.ContractTransaction;
    try {
      ({ tx, commitment } = await ensRegistrar.commit(
        name,
        salt,
        fee,
        signature,
        deadline
      ));
    } finally {
      commitNotification.remove();
    }
    ensRegistrar.persistCommitment(commitment);
    transaction.add(transaction.commitEnsName(tx));

    const txNotification = notification.info({
      message: "Waiting for the transaction to be included",
      showIcon: true,
      persist: true,
    });

    let receipt;
    try {
      receipt = await tx.wait(1);
    } finally {
      txNotification.remove();
    }

    return {
      type: "register",
      commitment,
      commitmentBlock: receipt.blockNumber,
    };
  }
</script>

{#if state.type === "validateAndCommit"}
  <Modal emoji="📇" title="Let’s name your org">
    <svelte:fragment slot="description">
      What should your org be called? This name will show up on the top of your
      profile and anywhere you interact as an org on Radicle.
    </svelte:fragment>

    <TextInput
      bind:value={nameInputValue}
      showSuccessCheck
      disabled={commitInProgress}
      validation={validationState}
      suffix={`.${ensResolver.DOMAIN}`}
      placeholder="Your org name"
      style="margin: 16px auto; width: 352px;" />

    <svelte:fragment slot="buttons">
      <Button
        variant="transparent"
        on:click={() => {
          modal.hide();
        }}>Cancel</Button>
      <Button
        on:click={commitOrGoToUpdateMetadata}
        disabled={validationState.status !==
          validation.ValidationStatus.Success || commitInProgress}
        >Continue</Button>
    </svelte:fragment>
  </Modal>
{:else if state.type === "register"}
  <ConfirmRegistration
    commitment={state.commitment}
    commitmentBlock={state.commitmentBlock}
    {registrationDone} />
{:else}
  {unreachable(state)}
{/if}
