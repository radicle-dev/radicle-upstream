<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";
  import * as error from "ui/src/error";
  import { unreachable } from "ui/src/unreachable";
  import { Emoji } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";
  import BlockTimer from "./BlockTimer.svelte";

  let buttonsDisabled = false;
  let confirmButtonCopy = "Confirm registration";

  export let done: () => void;
  export let name: string;
  export let commitmentSalt: Uint8Array;
  export let commitmentBlock: number;
  export let requiredBlocks: number;

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
  <div style="color: var(--color-foreground-level-5);">
    <BlockTimer
      onFinish={() => (state = "readyToRegister")}
      {requiredBlocks}
      startBlock={commitmentBlock} />
    <h3 style="margin-top: 24px">Awaiting registration commitment...</h3>
    <p style="margin: 24px 0">
      This will take about one minute. The waiting period is required to ensure
      another person hasn’t tried to register the same name.
    </p>
  </div>
{:else if state === "readyToRegister"}
  <Header
    title="Almost done"
    description={`With this last transaction, you’re confirming the ` +
      `registration of your new ENS name ` +
      `${name}.${ensResolver.DOMAIN}.`} />
  <ButtonRow
    disableButtons={buttonsDisabled}
    onSubmit={register}
    confirmCopy={confirmButtonCopy} />
{:else if state === "success"}
  <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
  <Header
    title="Registration complete"
    description={`Congratulations, ` +
      `${name}.${ensResolver.DOMAIN} has successfully been ` +
      `registered with your wallet. Next, let's populate your name with ` +
      `organization metadata.`} />
  <p
    style="color: var(--color-foreground-level-5; margin: 16px 0;"
    class="typo-text-small">
    You can also do this later by selecting "Register ENS Name" and entering
    your existing name.
  </p>
  <ButtonRow
    onSubmit={done}
    cancelCopy="Do this later"
    confirmCopy="Set organization metadata" />
{:else}
  {unreachable(state)}
{/if}
