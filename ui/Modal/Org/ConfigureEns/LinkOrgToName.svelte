<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as error from "ui/src/error";
  import * as org from "ui/src/org";

  import { Emoji, TextInput } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let onSubmit: () => void;
  export let domain: string;
  export let orgAddress: string;
  export let safeAddress: string | undefined = undefined;

  let buttonsDisabled = false;
  let submitButtonCopy = "Link organization to name";

  let linked = false;

  async function link() {
    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    try {
      if (safeAddress) {
        await org.proposeSetNameChange(domain, orgAddress, safeAddress);
      } else {
        await org.setNameSingleSig(domain, orgAddress);
      }

      linked = true;
    } catch (err) {
      buttonsDisabled = false;
      submitButtonCopy = "Link organization to name";

      throw new error.Error({
        message: "Transaction failed",
        source: err,
      });
    }
  }
</script>

<style>
  .label {
    padding-left: 12px;
    margin-bottom: 12px;
    color: var(--color-foreground-level-6);
  }
</style>

{#if !linked}
  <Emoji emoji="🔗" size="huge" style="margin-bottom: 16px" />
  <Header
    title="Let’s link your name"
    style="margin-bottom: 24px"
    description={`In this last step, we’re updating your organization to ` +
      `point towards your newly created name. Once that’s done, your ` +
      `organization will appear with your new name across Radicle!`} />

  <div class="label typo-text-bold">Organization address</div>
  <TextInput disabled style="margin-bottom: 24px" value={orgAddress} />

  <div class="label typo-text-bold">Name</div>
  <TextInput disabled style="margin-bottom: 24px" value={domain} />

  <p
    style="color: var(--color-foreground-level-5; margin: 16px 0;"
    class="typo-text-small">
    You can also do this later by selecting "Register ENS Name" and entering
    your existing name.
  </p>

  <ButtonRow
    disableButtons={buttonsDisabled}
    onSubmit={link}
    canCancel={false}
    confirmCopy={submitButtonCopy} />
{:else if safeAddress}
  <Emoji emoji="🤝" size="huge" style="margin-bottom: 16px" />
  <Header
    title="Approve on Gnosis"
    description={"As a final step your organisation will have to confirm " +
      "the transaction on Gnosis. After it's been approved and executed " +
      "your newly registered name will start appearing across Radicle in " +
      "place of your organization address!"} />
  <ButtonRow
    onSubmit={() => {
      safeAddress && org.openOnGnosisSafe(safeAddress, "transactions");
      onSubmit();
    }}
    canCancel={false}
    confirmCopy="View proposal on Gnosis" />
{:else}
  <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
  <Header
    title="That's it!"
    description={`Great, your organization now points to your new name ` +
      `${domain}. Shortly, your name ` +
      `will start appearing across Radicle in place of your organization ` +
      `address!`} />
  <ButtonRow {onSubmit} canCancel={false} confirmCopy="Amazing, thanks!" />
{/if}
