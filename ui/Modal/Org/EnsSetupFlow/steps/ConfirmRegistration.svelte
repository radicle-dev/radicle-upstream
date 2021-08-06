<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { register } from "ui/src/org/ensRegistrar";
  import type { EnsConfiguration } from "../ens-flow.types";
  import ButtonRow from "./shared/ButtonRow.svelte";
  import * as error from "ui/src/error";
  import * as wallet from "ui/src/wallet";
  import * as svelteStore from "ui/src/svelteStore";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

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

      await register(walletStore.environment, ensConfiguration.name, salt);

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
  <HeadlineAndDescription
    headline="Almost done"
    description={`With this last transaction, you’re confirming the registration of your new ENS name ${ensConfiguration.name}.radicle.eth.`} />
  <ButtonRow
    disableButtons={buttonsDisabled}
    onSubmit={handleSubmit}
    confirmCopy={confirmButtonCopy} />
</div>
