<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ethers from "ethers";

  import { unreachable } from "ui/src/unreachable";
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import * as svelteStore from "ui/src/svelteStore";
  import * as wallet from "ui/src/wallet";

  import { Modal } from "ui/DesignSystem";
  import ConfirmRegistration from "./ConfirmRegistration.svelte";
  import ButtonRow from "./shared/ButtonRow.svelte";

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
        minAge: number;
      };

  let state: State = { type: "commit" };
  let buttonsDisabled = false;
  let confirmButtonCopy = "Begin registration";

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
        minAge: commitResult.minAge,
        commitmentBlock: commitResult.receipt.blockNumber,
        commitmentSalt: salt,
      };
    } catch (err) {
      buttonsDisabled = false;
      confirmButtonCopy = "Begin registration";

      error.show(
        new error.Error({
          message:
            "Transaction failed. Please try again and confirm the signature & transaction in your connected wallet.",
          source: err,
        })
      );
    }
  }
</script>

{#if state.type === "commit"}
  <Modal
    emoji="📇"
    title="Let’s name your organization"
    desc={`${name}.${
      ensResolver.DOMAIN
    } is available for registration for ${ensRegistrar.formatFee(fee)} RAD.`}>
    <ButtonRow
      disableButtons={buttonsDisabled}
      confirmCopy={confirmButtonCopy}
      onSubmit={commit} />
  </Modal>
{:else if state.type === "register"}
  <ConfirmRegistration
    {name}
    commitmentSalt={state.commitmentSalt}
    commitmentBlock={state.commitmentBlock}
    minAge={state.minAge}
    {done} />
{:else}
  {unreachable(state)}
{/if}
