<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsConfiguration } from "./ens-flow.types";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  const walletStore = svelteStore.get(wallet.store);

  let buttonsDisabled = false;
  let confirmButtonCopy = "Confirm registration";

  export let onSubmit: () => void = () => {};
  export let ensConfiguration: EnsConfiguration;

  async function handleSubmit() {
    buttonsDisabled = true;
    confirmButtonCopy = "Waiting for transaction confirmation...";

    try {
      const salt = ensConfiguration.commitmentSalt;

      await ensRegistrar.register(
        walletStore.environment,
        ensConfiguration.name,
        salt
      );

      onSubmit();
    } catch (err) {
      buttonsDisabled = false;
      confirmButtonCopy = "Confirm registration";

      throw new error.Error({
        message: "Transaction failed",
        source: err,
      });
    }
  }
</script>

<div>
  <Header
    title="Almost done"
    description={`With this last transaction, you’re confirming the ` +
      `registration of your new ENS name ` +
      `${ensConfiguration.name}.${ensResolver.DOMAIN}.`} />
  <ButtonRow
    disableButtons={buttonsDisabled}
    onSubmit={handleSubmit}
    confirmCopy={confirmButtonCopy} />
</div>
