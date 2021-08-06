<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsMetadataPayload, SubmitPayload } from "./ens-flow.types";
  import type { ValidationState } from "ui/src/validation";

  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";
  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";
  import TextInput from "ui/DesignSystem/TextInput.svelte";
  import { ValidationStatus } from "ui/src/validation";
  import { checkAvailability } from "ui/src/org/ensRegistrar";
  import { getRegistration } from "ui/src/org/ensResolver";

  export let onSubmit: (payload: SubmitPayload) => void = () => {};
  export let ensMetadataConfiguration: EnsMetadataPayload | undefined;

  let name: string;
  if (ensMetadataConfiguration && ensMetadataConfiguration.name) {
    const existingName = ensMetadataConfiguration.name.replace(
      ".radicle.eth",
      ""
    );
    if (existingName.length > 0) {
      name = existingName;
    }
  }

  let validationStatus: ValidationState = {
    status: ValidationStatus.NotStarted,
  };

  const walletStore = svelteStore.get(wallet.store);

  async function handleSubmit() {
    if (!name) {
      validationStatus = {
        status: ValidationStatus.Error,
        message: "This field is required.",
      };
      return;
    }

    validationStatus = {
      status: ValidationStatus.Loading,
    };

    const { available, fee } = await checkAvailability(
      walletStore.environment,
      name
    );

    if (available) {
      onSubmit({
        ensNameConfiguration: {
          name,
          fee,
        },
      });
    } else {
      const registration = await getRegistration(`${name}.radicle.eth`);

      if (registration && registration.owner === walletStore.getAddress()) {
        onSubmit({
          ensNameConfiguration: {
            name,
            address: walletStore.getAddress(),
            registered: true,
          },
          ensMetadata: {
            ...registration,
          },
        });
      }

      validationStatus = {
        status: ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      };
    }
  }

  // Reset validation when user changes input
  $: {
    name;
    validationStatus = {
      status: ValidationStatus.NotStarted,
    };
  }
</script>

<div>
  <HeadlineAndDescription
    headline="Let’s name your organization"
    description="What should your organization be called?" />
  <TextInput
    bind:value={name}
    validation={validationStatus}
    suffix=".radicle.eth"
    placeholder="Your organization name"
    style="margin: 16px auto; width: 352px;" />
  <ButtonRow onSubmit={handleSubmit} confirmCopy="Continue" />
</div>
