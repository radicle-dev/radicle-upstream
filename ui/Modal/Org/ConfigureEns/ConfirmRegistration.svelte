<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as registerName from "./RegisterName.svelte";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import { unreachable } from "ui/src/unreachable";
  import { Modal } from "ui/DesignSystem";

  import ButtonRow from "./ButtonRow.svelte";
  import BlockTimer from "./BlockTimer.svelte";

  let buttonsDisabled = false;
  let confirmButtonCopy = "Confirm registration";

  export let registrationDone: (result: registerName.Result) => void;
  export let name: string;
  export let commitmentSalt: Uint8Array;
  export let commitmentBlock: number;
  export let minAge: number;

  let state: "waiting" | "readyToRegister" | "success" = "waiting";

  async function register() {
    buttonsDisabled = true;
    confirmButtonCopy = "Waiting for transaction confirmation...";

    try {
      await ensRegistrar.register(name, commitmentSalt);

      state = "success";
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

{#if state === "waiting"}
  <Modal
    emoji="📇"
    title="Awaiting registration commitment"
    desc="This will take about one minute. The waiting period is required to ensure another person hasn’t tried to register the same name.">
    <div style="display: flex; justify-content: center;">
      <BlockTimer
        onFinish={() => (state = "readyToRegister")}
        {minAge}
        startBlock={commitmentBlock} />
    </div>
  </Modal>
{:else if state === "readyToRegister"}
  <Modal
    emoji="📇"
    title="Almost done"
    desc={`With this last transaction, you’re confirming the registration of your new ENS name ${name}.${ensResolver.DOMAIN}.`}>
    <ButtonRow
      disableButtons={buttonsDisabled}
      onSubmit={register}
      confirmCopy={confirmButtonCopy} />
  </Modal>
{:else if state === "success"}
  <Modal
    emoji="🎉"
    title="Registration complete"
    desc={`Congratulations, ${name}.${ensResolver.DOMAIN} has successfully been registered with your wallet. Next, let's populate your name with organization metadata. You can also do this later by selecting "Register ENS Name" and entering your existing name.`}>
    <ButtonRow
      onSubmit={() => {
        registrationDone({ name, registration: null });
      }}
      cancelCopy="Do this later"
      confirmCopy="Set organization metadata" />
  </Modal>
{:else}
  {unreachable(state)}
{/if}
