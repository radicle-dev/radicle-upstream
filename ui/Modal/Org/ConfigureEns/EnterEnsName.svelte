<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript" context="module">
  export interface Result {
    registration?: ensResolver.Registration;
    name: string;
  }
</script>

<script lang="typescript">
  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import { TextInput } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as validation from "ui/src/validation";

  export let onSubmit: (result: Result) => void;
  export let currentName: string | undefined;

  let name = currentName;

  let validationStatus: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

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

    const available = await ensRegistrar.isAvailable(name);

    if (available) {
      onSubmit({
        name,
      });
    } else {
      const registration = await ensResolver.getRegistration(
        `${name}.${ensResolver.DOMAIN}`
      );

      const walletStore = svelteStore.get(wallet.store);

      if (registration && registration.owner === walletStore.getAddress()) {
        onSubmit({
          registration,
          name,
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
