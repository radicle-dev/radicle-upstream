<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";

  import { Modal, TextInput, Tooltip } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";

  export let onSubmit: () => void;
  export let registration: ensResolver.Registration;
  export let orgAddress: string;

  let urlValue: string | undefined;
  let avatarValue: string | undefined;
  let twitterValue: string | undefined;
  let githubValue: string | undefined;

  let updated = false;
  let buttonsDisabled = false;
  let submitButtonCopy = "Update name metadata";

  $: {
    urlValue = registration.url || undefined;
    avatarValue = registration.avatar || undefined;
    twitterValue = registration.twitter || undefined;
    githubValue = registration.github || undefined;
  }

  async function setRecords() {
    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    try {
      let records: {
        name: keyof ensResolver.Registration;
        value: string | undefined;
      }[] = [
        { name: "address", value: orgAddress },
        { name: "url", value: urlValue },
        { name: "avatar", value: avatarValue },
        { name: "twitter", value: twitterValue },
        { name: "github", value: githubValue },
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
        await ensResolver.setRecords(
          registration.domain,
          records as ensResolver.EnsRecord[]
        );
      }

      updated = true;
    } catch (err) {
      buttonsDisabled = false;
      submitButtonCopy = "Update name metadata";
      throw new error.Error({
        message: "Transaction failed",
        source: err,
      });
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

{#if !updated}
  <Modal
    emoji="📇"
    title="Set your name’s metadata"
    desc={"This will be shown alongside your ENS name, and appears together with your org across Radicle. You can edit it at any time by clicking “Edit ENS name” on the org page."}>
    <div class="label typo-text-bold">Organization address</div>
    <Tooltip
      value={"This is the address of your organization and is required to link your ENS name to it."}
      position="top">
      <TextInput style="margin-bottom: 24px" disabled value={orgAddress} />
    </Tooltip>

    <div class="label typo-text-bold">Website URL</div>
    <TextInput
      style="margin-bottom: 24px"
      placeholder="https://radicle.xyz/"
      bind:value={urlValue} />

    <div class="label typo-text-bold">Avatar URL</div>
    <TextInput
      style="margin-bottom: 24px"
      placeholder="https://radicle.xyz/logo.png"
      bind:value={avatarValue} />

    <div class="label typo-text-bold">Twitter username</div>
    <TextInput
      style="margin-bottom: 24px"
      placeholder="@radicle"
      bind:value={twitterValue} />

    <div class="label typo-text-bold">GitHub username</div>
    <TextInput
      style="margin-bottom: 24px"
      placeholder="radicle-dev"
      bind:value={githubValue} />

    <ButtonRow
      disableButtons={buttonsDisabled}
      confirmCopy={submitButtonCopy}
      onSubmit={setRecords} />
  </Modal>
{:else}
  <Modal
    emoji="🎉"
    title="Metadata successfully updated"
    desc={`Great, your name ${registration.domain} has successfully been updated with your supplied metadata.`}>
    <ButtonRow {onSubmit} canCancel={false} confirmCopy="Continue" />
  </Modal>
{/if}
