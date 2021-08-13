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
  import type * as ethers from "ethers";
  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import { Modal, TextInput } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as validation from "ui/src/validation";

  export let onSubmit: (result: Result) => void;
  export let currentName: string | undefined;
  export let fee: ethers.BigNumber;

  let name = currentName || "";

  let validationStatus: validation.ValidationState = {
    status: validation.ValidationStatus.NotStarted,
  };

  let timeoutHandle: NodeJS.Timeout;
  let userInputStarted: boolean = name !== "";

  function validateName(name: string | undefined): void {
    if (!userInputStarted) {
      userInputStarted = true;
      return;
    }

    if (!name) {
      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "You need to enter a name.",
      };
    } else {
      validationStatus = {
        status: validation.ValidationStatus.Loading,
      };
      if (timeoutHandle) {
        clearTimeout(timeoutHandle);
      }
      timeoutHandle = setTimeout(() => {
        checkNameAvailability();
      }, 1000);
    }
  }

  async function checkNameAvailability(): Promise<void> {
    const available = await ensRegistrar.isAvailable(name);

    if (available) {
      const accountBalancesStore = wallet.accountBalancesStore;
      const radBalance = svelteStore.get(accountBalancesStore).rad;

      if (radBalance && radBalance < fee) {
        validationStatus = {
          status: validation.ValidationStatus.Error,
          message:
            "You don't have enough RAD in your wallet to register this name.",
        };

        return;
      }

      // Show the green checkmark on the right side of the input box.
      validationStatus = { status: validation.ValidationStatus.Success };
    } else {
      const registration = await ensResolver.getRegistration(
        `${name}.${ensResolver.DOMAIN}`
      );

      const walletStore = svelteStore.get(wallet.store);

      if (registration && registration.owner === walletStore.getAddress()) {
        validationStatus = { status: validation.ValidationStatus.Success };
        return;
      }

      validationStatus = {
        status: validation.ValidationStatus.Error,
        message: "Sorry, but that name is already taken.",
      };
    }
  }

  async function handleSubmit(): Promise<void> {
    /*   const walletStore = svelteStore.get(wallet.store); */
    /*   const registration = await ensResolver.getRegistration( */
    /*     `${name}.${ensResolver.DOMAIN}` */
    /*   ); */
    /*   if (registration && registration.owner === walletStore.getAddress()) { */
    /*     onSubmit({ */
    /*       registration, */
    /*       name, */
    /*     }); */
    /*   } else { */
    /*     onSubmit({ */
    /*       name, */
    /*     }); */
    /*   } */
  }

  $: validateName(name);
</script>

<Modal
  emoji="📇"
  title="Let’s name your org"
  desc="What should your org be called? This name will show up on the top of your profile and anywhere you interact as an org on Radicle.">
  <TextInput
    bind:value={name}
    showSuccessCheck
    validation={validationStatus}
    suffix={`.${ensResolver.DOMAIN}`}
    placeholder="Your organization name"
    style="margin: 16px auto; width: 352px;" />

  <ButtonRow
    onSubmit={handleSubmit}
    confirmCopy="Continue"
    disableButtons={validationStatus.status !==
      validation.ValidationStatus.Success} />
</Modal>
