<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as configureEns from "ui/src/org/configureEns";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as org from "ui/src/org";
  import * as transaction from "ui/src/transaction";

  import Button from "design-system/Button.svelte";
  import TextInput from "design-system/TextInput.svelte";

  import Modal from "ui/App/ModalLayout/Modal.svelte";

  export let onSubmit: () => void;
  export let domain: string;
  export let orgAddress: string;
  export let safeAddress: string | undefined = undefined;

  let continueButtonDisabled = false;

  async function link() {
    continueButtonDisabled = true;

    if (safeAddress) {
      const signNotification = notification.show({
        type: "info",
        message:
          "Waiting for you to sign the proposal in your connected wallet",
        persist: true,
      });

      try {
        await org.proposeSetNameChange(domain, orgAddress, safeAddress);
        notification.show({
          type: "info",
          message:
            "Your org metadata will be updated once the quorum of members have confirmed the transaction",
          persist: true,
          actions: [
            {
              label: "View on Gnosis Safe",
              handler: () => {
                safeAddress &&
                  org.openOnGnosisSafe(safeAddress, "transactions/queue");
              },
            },
            {
              label: "Dismiss",
              handler: () => {},
            },
          ],
        });
        onSubmit();
      } catch (err: unknown) {
        continueButtonDisabled = false;

        notification.showException(error.fromUnknown(err));
      } finally {
        signNotification.remove();
      }
    } else {
      const setNameNotification = notification.show({
        type: "info",
        message:
          "Waiting for you to confirm the set name transaction in your connected wallet",
        persist: true,
      });

      let tx: transaction.ContractTransaction | undefined = undefined;
      let waitingForTxNotification;

      try {
        tx = await org.setNameSingleSig(domain, orgAddress);
        transaction.add(transaction.linkEnsNameToOrg(tx));
        waitingForTxNotification = notification.show({
          type: "info",
          message: `Once the transaction has been included, your org will point to ${domain}`,
          persist: true,
        });
        onSubmit();
      } catch (err: unknown) {
        continueButtonDisabled = false;

        notification.showException(error.fromUnknown(err));
        return;
      } finally {
        setNameNotification.remove();
      }

      try {
        await tx.wait(1);
      } finally {
        waitingForTxNotification.remove();
      }

      await configureEns.updateScreenAndNotifyUser(
        orgAddress,
        `Your org ${orgAddress} now points to ${domain}`
      );
    }
  }
</script>

<style>
  .label {
    padding-left: 12px;
    margin-bottom: 12px;
    color: var(--color-foreground-level-6);
  }
</style>

<Modal emoji="🔗" title="Let’s link your name">
  <svelte:fragment slot="description">
    In this last step, we’ll update your org to point to your newly created
    name. Once that’s done, your org will appear with your new name across
    Radicle!
  </svelte:fragment>

  <div class="label typo-text-bold">Org address</div>
  <TextInput
    disabled
    style="width: auto; margin-bottom: 24px"
    value={orgAddress} />

  <div class="label typo-text-bold">Name</div>
  <TextInput disabled style="width: auto; margin-bottom: 24px" value={domain} />

  <p
    style="color: var(--color-foreground-level-5; margin: 16px 0;"
    class="typo-text-small">
    You can also do this later by selecting “Register ENS Name” from the
    dropdown on your org’s profile and entering your existing name.
  </p>

  <svelte:fragment slot="buttons">
    <Button
      variant="transparent"
      on:click={() => {
        modal.hide();
      }}>Cancel</Button>
    <Button on:click={link} disabled={continueButtonDisabled}
      >Link name to org</Button>
  </svelte:fragment>
</Modal>
