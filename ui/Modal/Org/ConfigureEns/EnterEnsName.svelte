<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsMetadataPayload, SubmitPayload } from "./ens-flow.types";

  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import { TextInput } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as validation from "ui/src/validation";

  export let onSubmit: (payload: SubmitPayload) => void = () => {};
  export let ensMetadataConfiguration: EnsMetadataPayload | undefined;

  let name: string;
  if (ensMetadataConfiguration && ensMetadataConfiguration.name) {
    const existingName = ensMetadataConfiguration.name.replace(
      `.${ensResolver.DOMAIN}`,
      ""
    );
    if (existingName.length > 0) {
      name = existingName;
    }
  }

  let validationStatus: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

  const walletStore = svelteStore.get(wallet.store);

  async function handleSubmit() {
    if (!name) {
      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "This field is required.",
      };
      return;
    }

    validationStatus = {
      status: validation.ValidationStatus.Loading,
    };

    const { available, fee } = await ensRegistrar.checkAvailability(
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
      const registration = await ensResolver.getRegistration(
        `${name}.${ensResolver.DOMAIN}`
      );

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
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      };
    }
  }

  // Reset validation when user changes input
  $: {
    name;
    validationStatus = {
      status: validation.ValidationStatus.NotStarted,
    };
  }
</script>

<div>
  <Header
    title="Let’s name your organization"
    description="What should your organization be called?" />
  <TextInput
    bind:value={name}
    validation={validationStatus}
    suffix={`.${ensResolver.DOMAIN}`}
    placeholder="Your organization name"
    style="margin: 16px auto; width: 352px;" />
  <ButtonRow onSubmit={handleSubmit} confirmCopy="Continue" />
</div>
