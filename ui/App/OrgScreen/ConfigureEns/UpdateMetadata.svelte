<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "design-system/TextInput";

  import * as configureEns from "ui/src/org/configureEns";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as transaction from "ui/src/transaction";

  import Button from "design-system/Button.svelte";
  import TextInput from "design-system/TextInput.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  import Modal from "ui/App/ModalLayout/Modal.svelte";

  export let onSubmit: () => void;
  export let registration: ensResolver.Registration;
  export let orgAddress: string;

  let urlValue: string | undefined = registration.url || undefined;
  let avatarValue: string | undefined = registration.avatar || undefined;
  let twitterValue: string | undefined = registration.twitter || undefined;
  let githubValue: string | undefined = registration.github || undefined;
  let seedIdValue: string | undefined = registration.seedId || undefined;
  let seedHostValue: string | undefined = registration.seedHost || undefined;

  let setRecordsInProgress = false;

  let orgAddressValidationState: TextInputValidationState = {
    type: "unvalidated",
  };

  if (
    registration.address &&
    registration.address.toLowerCase() !== orgAddress.toLowerCase()
  ) {
    orgAddressValidationState = {
      type: "invalid",
      message: `This name already points to your org with the address ${registration.address}. If you change this here, it will overwrite the existing metadata associated with this ENS name.`,
    };
  }

  async function setRecords() {
    setRecordsInProgress = true;

    let records: {
      name: keyof ensResolver.Registration;
      value: string | undefined;
    }[] = [
      { name: "address", value: orgAddress },
      { name: "url", value: urlValue },
      { name: "avatar", value: avatarValue },
      { name: "twitter", value: twitterValue },
      { name: "github", value: githubValue },
      { name: "seedId", value: seedIdValue },
      { name: "seedHost", value: seedHostValue },
    ];

    // Filter out unchanged records.
    records = records.filter(r => {
      const existingValue = registration[r.name];

      const normalizedExistingValue =
        typeof existingValue === "string"
          ? existingValue.toLowerCase()
          : existingValue;

      if (
        r.value === undefined ||
        (normalizedExistingValue === null && r.value === "")
      ) {
        false;
      } else {
        return normalizedExistingValue !== r.value.toLowerCase();
      }
    });

    if (records.length > 0) {
      const updateNotification = notification.show({
        type: "info",
        message:
          "Waiting for you to confirm the metadata update transaction in your connected wallet",
        persist: true,
      });

      let tx: transaction.ContractTransaction | undefined = undefined;
      let waitingForTxNotification;

      try {
        tx = await ensResolver.setRecords(
          registration.domain,
          records as ensResolver.EnsRecord[]
        );
        transaction.add(transaction.updateEnsMetadata(tx));
        waitingForTxNotification = notification.show({
          type: "info",
          message:
            "The org’s updated metadata will appear once the transaction has been included",
          persist: true,
        });
        onSubmit();
      } catch (err: unknown) {
        setRecordsInProgress = false;

        notification.showException(error.fromUnknown(err));
        return;
      } finally {
        updateNotification.remove();
      }

      try {
        await tx.wait(1);
      } finally {
        waitingForTxNotification.remove();
      }

      await configureEns.updateScreenAndNotifyUser(
        orgAddress,
        "Your org’s metadata has been updated"
      );
    } else {
      onSubmit();
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

<Modal emoji="📋" title="Set your org’s metadata">
  <svelte:fragment slot="description">
    This will be shown alongside your ENS name, and appears together with your
    org across Radicle. You can edit it at any time by clicking “Edit ENS name”
    on the org page.
  </svelte:fragment>

  <div class="label typo-text-bold">Org address</div>
  <Tooltip
    value={"This is the address of your org and is required to link your ENS name to it."}
    position="top">
    <TextInput
      style="margin-bottom: 24px"
      disabled
      value={orgAddress}
      validationState={orgAddressValidationState} />
  </Tooltip>

  <div class="label typo-text-bold">Website URL</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="The URL to your org’s website"
    bind:value={urlValue} />

  <div class="label typo-text-bold">Avatar URL</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="A URL that points to the avatar for your org"
    bind:value={avatarValue} />

  <div class="label typo-text-bold">Twitter username</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="Your org’s Twitter handle"
    bind:value={twitterValue} />

  <div class="label typo-text-bold">GitHub username</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="Your org’s GitHub username"
    bind:value={githubValue} />

  <div class="label typo-text-bold">Seed ID</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="The Peer ID of the seed"
    bind:value={seedIdValue} />

  <div class="label typo-text-bold">Seed Host</div>
  <TextInput
    disabled={setRecordsInProgress}
    style="margin-bottom: 24px"
    placeholder="The seed host name"
    bind:value={seedHostValue} />

  <svelte:fragment slot="buttons">
    <Button
      variant="transparent"
      on:click={() => {
        modal.hide();
      }}>Cancel</Button>
    <Button on:click={setRecords} disabled={setRecordsInProgress}
      >Update org metadata</Button>
  </svelte:fragment>
</Modal>
