<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as registerName from "./RegisterName.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as transaction from "ui/src/transaction";

  import { Button, Modal } from "ui/DesignSystem";

  import BlockTimer from "./BlockTimer.svelte";

  let confirmButtonDisabled = true;

  export let commitment: ensRegistrar.Commitment;
  export let commitmentBlock: number;
  export let registrationDone: (result: registerName.Result) => void;

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
      registrationTx = await ensRegistrar.register(
        commitment.name,
        commitment.salt
      );
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
    ensRegistrar.clearCommitment();

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

    notification.info({
      message: `${commitment.name}.${ensResolver.DOMAIN} has been registered with your wallet`,
      showIcon: true,
    });
    registrationDone({ name: commitment.name, registration: undefined });
  }
</script>

<Modal emoji="📇" title="Awaiting registration commitment">
  <svelte:fragment slot="description">
    The waiting period is required to ensure another person hasn’t tried to
    register the same name.
  </svelte:fragment>

  <div style="display: flex; justify-content: center;">
    <BlockTimer
      onFinish={() => {
        confirmButtonDisabled = false;
      }}
      startBlock={commitmentBlock}
      minimumCommitmentAge={commitment.minimumCommitmentAge} />
  </div>

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
