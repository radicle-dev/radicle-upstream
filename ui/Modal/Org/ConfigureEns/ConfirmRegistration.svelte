<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as registerName from "./RegisterName.svelte";

  import { unreachable } from "ui/src/unreachable";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as transaction from "ui/src/transaction";

  import { Button, Modal } from "ui/DesignSystem";

  import BlockTimer from "./BlockTimer.svelte";

  let confirmButtonDisabled = false;

  export let registrationDone: (result: registerName.Result) => void;
  export let name: string;
  export let commitmentSalt: Uint8Array;
  export let commitmentBlock: number;
  export let minimumCommitmentAge: number;

  let state: "waiting" | "readyToRegister" | "success" = "waiting";

  async function register() {
    confirmButtonDisabled = true;

    const registrationNotification = notification.info({
      message:
        "Waiting for you to confirm the registration transaction in your connected wallet",
      showIcon: true,
      persist: true,
    });

    let registrationTx: transaction.ContractTransaction;
    try {
      registrationTx = await ensRegistrar.register(name, commitmentSalt);
    } catch (err) {
      confirmButtonDisabled = false;

      error.show(
        new error.Error({
          message: err.message,
          source: err,
        })
      );
      // Don't advance flow if the user rejected the tx.
      return;
    } finally {
      registrationNotification.remove();
    }

    transaction.add(transaction.registerEnsName(registrationTx));

    const txNotification = notification.info({
      message: "Waiting for the transaction to be included",
      showIcon: true,
      persist: true,
    });

    try {
      await registrationTx.wait(1);
    } catch (err) {
      confirmButtonDisabled = false;

      error.show(
        new error.Error({
          message: err.message,
          source: err,
        })
      );
      // Don't advance flow unless we have the tx receipt.
      return;
    } finally {
      txNotification.remove();
    }

    state = "success";
  }
</script>

{#if state === "waiting"}
  <Modal emoji="📇" title="Awaiting registration commitment">
    <svelte:fragment slot="description">
      This will take about one minute. The waiting period is required to ensure
      another person hasn’t tried to register the same name.
    </svelte:fragment>

    <div style="display: flex; justify-content: center;">
      <BlockTimer
        onFinish={() => (state = "readyToRegister")}
        {minimumCommitmentAge}
        startBlock={commitmentBlock} />
    </div>
  </Modal>
{:else if state === "readyToRegister"}
  <Modal emoji="📇" title="Almost done">
    <svelte:fragment slot="description">
      With this last transaction, you’re confirming the registration of your new
      ENS name <span class="typo-text-bold">{name}.{ensResolver.DOMAIN}</span>.
    </svelte:fragment>

    <svelte:fragment slot="buttons">
      <Button
        variant="transparent"
        on:click={() => {
          modal.hide();
        }}>Cancel</Button>
      <Button on:click={register} disabled={confirmButtonDisabled}
        >Confirm registration</Button>
    </svelte:fragment>
  </Modal>
{:else if state === "success"}
  <Modal emoji="🎉" title="Registration complete">
    <svelte:fragment slot="description">
      Congratulations, <span class="typo-text-bold"
        >{name}.{ensResolver.DOMAIN}</span> has been registered with your wallet.
      Next, let’s populate your name with org metadata. You can also do this later
      by selecting “Register ENS Name” in the menu on your org’s profile and entering
      your existing name.
    </svelte:fragment>

    <svelte:fragment slot="buttons">
      <Button
        variant="transparent"
        on:click={() => {
          modal.hide();
        }}>Do this later</Button>
      <Button
        on:click={() => {
          registrationDone({ name, registration: null });
        }}>Set org metadata</Button>
    </svelte:fragment>
  </Modal>
{:else}
  {unreachable(state)}
{/if}
