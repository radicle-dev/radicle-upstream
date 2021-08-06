<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { BigNumber } from "@ethersproject/bignumber";
  import { ethers } from "ethers";
  import type { EnsConfiguration, SubmitPayload } from "./ens-flow.types";
  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";
  import * as wallet from "ui/src/wallet";
  import * as svelteStore from "ui/src/svelteStore";
  import { commit } from "ui/src/org/ensRegistrar";
  import * as error from "ui/src/error";

  const walletStore = svelteStore.get(wallet.store);

  export let onSubmit: (payload: SubmitPayload) => void = () => {};
  export let ensConfiguration: EnsConfiguration;

  let buttonsDisabled = false;
  let confirmButtonCopy = "Begin registration";

  function formatFee(fee: BigNumber) {
    return ethers.utils.commify(
      parseFloat(ethers.utils.formatUnits(fee)).toFixed(2)
    );
  }

  async function handleSubmit() {
    buttonsDisabled = true;
    confirmButtonCopy = "Waiting for transaction confirmation...";

    try {
      const salt = ethers.utils.randomBytes(32);

      const commitResult = await commit(
        walletStore.environment,
        ensConfiguration.name,
        salt,
        ensConfiguration.fee
      );

      onSubmit({
        ensNameConfiguration: {
          minAge: commitResult.minAge,
          commitmentBlock: commitResult.receipt.blockNumber,
          commitmentSalt: salt,
        },
      });
    } catch (err) {
      buttonsDisabled = false;
      confirmButtonCopy = "Begin registration";

      throw new error.Error({
        message:
          "Transaction failed. Please try again and confirm the signature & transaction in your connected wallet.",
        source: err,
      });
    }
  }
</script>

<div>
  <HeadlineAndDescription
    headline="Let’s name your organization"
    description={`${
      ensConfiguration.name
    }.radicle.eth is available for registration for ${formatFee(
      ensConfiguration.fee
    )} RAD.`} />
  <ButtonRow
    disableButtons={buttonsDisabled}
    confirmCopy={confirmButtonCopy}
    onSubmit={handleSubmit} />
</div>
