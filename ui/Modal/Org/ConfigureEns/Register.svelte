<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ethers from "ethers";

  import { unreachable } from "ui/src/unreachable";
  import * as wallet from "ui/src/wallet";
  import * as svelteStore from "ui/src/svelteStore";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";

  import ConfirmRegistration from "./ConfirmRegistration.svelte";
  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let done: () => void;
  export let name: string;
  export let fee: ethers.BigNumber;

  type State =
    | {
        type: "commit";
      }
    | {
        type: "register";
        commitmentSalt: Uint8Array;
        commitmentBlock: number;
        requiredBlocks: number;
      };

  let state: State = { type: "commit" };
  let buttonsDisabled = false;
  let confirmButtonCopy = "Begin registration";

  function formatFee(fee: ethers.BigNumber) {
    return ethers.utils.commify(
      parseFloat(ethers.utils.formatUnits(fee)).toFixed(2)
    );
  }

  async function commit() {
    buttonsDisabled = true;
    confirmButtonCopy = "Waiting for transaction confirmation...";

    try {
      const salt = ethers.utils.randomBytes(32);

      const walletStore = svelteStore.get(wallet.store);

      const commitResult = await ensRegistrar.commit(
        walletStore.environment,
        name,
        salt,
        fee
      );

      state = {
        type: "register",
        requiredBlocks: commitResult.minAge,
        commitmentBlock: commitResult.receipt.blockNumber,
        commitmentSalt: salt,
      };
    } catch (err) {
      buttonsDisabled = false;
      confirmButtonCopy = "Begin registration";

      error.show(
        new error.Error({
          message:
            "Transaction failed. Please try again and confirm the signature & " +
            "transaction in your connected wallet.",
          source: err,
        })
      );
    }
  }
</script>

{#if state.type === "commit"}
  <Header
    title="Let’s name your organization"
    description={`${name}.${ensResolver.DOMAIN} is ` +
      `available for registration for ${formatFee(fee)} ` +
      `RAD.`} />
  <ButtonRow
    disableButtons={buttonsDisabled}
    confirmCopy={confirmButtonCopy}
    onSubmit={commit} />
{:else if state.type === "register"}
  <ConfirmRegistration
    {name}
    commitmentSalt={state.commitmentSalt}
    commitmentBlock={state.commitmentBlock}
    requiredBlocks={state.requiredBlocks}
    {done} />
{:else}
  {unreachable(state)}
{/if}
