<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type {
    EnsConfiguration,
    EnsMetadataPayload,
    SubmitPayload,
  } from "./ens-flow.types";
  import type { ValidationState } from "ui/src/validation";

  import { onMount } from "svelte";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as style from "ui/src/style";
  import * as svelteStore from "ui/src/svelteStore";
  import * as validation from "ui/src/validation";
  import * as wallet from "ui/src/wallet";

  import { TextInput, Tooltip } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  const walletStore = svelteStore.get(wallet.store);

  export let onSubmit: (payload: SubmitPayload) => void = () => {};
  export let ensMetadataConfiguration: EnsMetadataPayload | undefined;
  export let ensConfiguration: EnsConfiguration | undefined;
  export let orgAddress: string;

  const name: string | undefined =
    ensConfiguration?.name || ensMetadataConfiguration?.name || undefined;
  const addressValue: string | undefined = orgAddress || undefined;
  let urlValue: string | undefined;
  let avatarValue: string | undefined;
  let twitterValue: string | undefined;
  let githubValue: string | undefined;

  let buttonsDisabled = true;
  let submitButtonCopy = "Update name metadata";

  let validationStatus: ValidationState = {
    status: validation.ValidationStatus.Loading,
  };

  let addressValidationStatus: ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

  let registration: ensResolver.Registration;

  onMount(async () => {
    if (!ensConfiguration?.name && !ensMetadataConfiguration?.name) {
      throw new error.Error({
        message:
          "Expected at least one of ensNameConfiguration or ensMetadataConfiguration props",
        details: { ensConfiguration },
      });
    }

    /*
    When a name just got newly created, we need to fetch its ENS record
    and create a resolver in order to be able to update it. If we're
    editing an existing name, the resolver & rest of the registration are
    already populated so we can skip fetching.
    */
    if (ensConfiguration && !ensMetadataConfiguration?.resolver) {
      const registrationLookup = await ensResolver.getRegistration(
        `${ensConfiguration.name}.${ensResolver.DOMAIN}`
      );

      if (!registrationLookup) {
        throw new error.Error({
          message: "Couldn't fetch registration",
          details: { ensConfiguration },
        });
      }

      registration = registrationLookup;
    } else {
      if (!ensMetadataConfiguration) {
        throw new error.Error({
          message: "Expected ensMetadataConfiguration to be populated",
        });
      }

      registration = ensMetadataConfiguration;
    }

    if (registration.owner !== walletStore.getAddress()) {
      throw new error.Error({
        message: "You don't own this name",
        details: {
          owner: registration.owner,
          walletAddress: walletStore.getAddress(),
        },
      });
    }

    if (
      registration.address &&
      registration.address?.toLowerCase() !== addressValue?.toLowerCase()
    ) {
      addressValidationStatus = {
        status: validation.ValidationStatus.Error,
        message: `This name is currently pointing to an organization with address ${registration.address}.
          Updating metadata will overwrite the existing organization link for this name.`,
      };
    }

    buttonsDisabled = false;
    validationStatus = {
      status: validation.ValidationStatus.NotStarted,
    };
  });

  // Update field values whenever these here change.
  $: {
    ensMetadataConfiguration;
    ensConfiguration;
    updateFields();
  }

  function updateFields() {
    urlValue = ensMetadataConfiguration?.url || undefined;
    avatarValue = ensMetadataConfiguration?.avatar || undefined;
    twitterValue = ensMetadataConfiguration?.twitter || undefined;
    githubValue = ensMetadataConfiguration?.github || undefined;
  }

  async function handleSubmit() {
    if (!addressValue) {
      throw new error.Error({
        message: "Missing address field value",
      });
    }

    if (!name) {
      throw new error.Error({
        message: "Name is undefined",
      });
    }

    if (!registration.resolver) {
      throw new error.Error({
        message: "Missing ENS resolver",
      });
    }

    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    try {
      let records: {
        name: keyof ensResolver.Registration;
        value: string | undefined;
      }[] = [
        { name: "address", value: addressValue },
        { name: "url", value: urlValue },
        { name: "avatar", value: avatarValue },
        { name: "twitter", value: twitterValue },
        { name: "github", value: githubValue },
      ];

      // Filter out unchanged records.
      records = records.filter(r => {
        if (!ensMetadataConfiguration) {
          throw new error.Error({
            message: "ensMetadataConfiguration is undefined",
          });
        }

        const existingValue = ensMetadataConfiguration[r.name];

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
          name,
          registration.resolver,
          records as ensResolver.EnsRecord[]
        );
      }

      onSubmit({
        ensMetadata: {
          ...ensMetadataConfiguration,
          address: addressValue,
          url: urlValue,
          avatar: avatarValue,
          twitter: twitterValue,
          github: githubValue,
        },
      });
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

<div>
  <Header
    title="Set your name's metadata"
    description={"This following information will be saved alongside your " +
      "ENS name, and appears together with your organization across Radicle " +
      "once linked. You can edit it any time by clicking “Edit ENS name” on " +
      "the organization page."}
    style="margin-bottom: 24px" />

  <Tooltip
    value={"This is the address of your organization and is required to " +
      "link your ENS name to it."}
    position={style.CSSPosition.Top}>
    <TextInput
      label="Organization address"
      style="margin-bottom: 24px"
      disabled
      value={addressValue}
      validation={addressValidationStatus} />
  </Tooltip>

  <TextInput
    label="Website URL"
    style="margin-bottom: 24px"
    placeholder="https://radicle.xyz/"
    bind:value={urlValue}
    validation={validationStatus} />

  <TextInput
    label="Avatar URL"
    style="margin-bottom: 24px"
    placeholder="https://radicle.xyz/logo.png"
    bind:value={avatarValue}
    validation={validationStatus} />

  <TextInput
    label="Twitter username"
    style="margin-bottom: 24px"
    placeholder="@radicle"
    bind:value={twitterValue}
    validation={validationStatus} />

  <TextInput
    label="GitHub username"
    style="margin-bottom: 24px"
    placeholder="radicle-dev"
    bind:value={githubValue}
    validation={validationStatus} />

  <ButtonRow
    disableButtons={buttonsDisabled}
    confirmCopy={submitButtonCopy}
    onSubmit={handleSubmit} />
</div>
