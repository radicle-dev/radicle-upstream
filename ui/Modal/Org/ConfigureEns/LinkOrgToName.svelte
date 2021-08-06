<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as error from "ui/src/error";
  import Emoji from "ui/DesignSystem/Emoji.svelte";
  import TextInput from "ui/DesignSystem/TextInput.svelte";
  import * as org from "ui/src/org";

  import type { EnsConfiguration, EnsMetadataPayload } from "./ens-flow.types";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

  export let onSubmit: () => void = () => {};
  export let ensConfiguration: EnsConfiguration;
  export let ensMetadataConfiguration: EnsMetadataPayload;
  export let safeAddress: string | undefined = undefined;

  let buttonsDisabled = false;
  let submitButtonCopy = "Link organization to name";

  async function linkSingleSig() {
    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    if (!ensConfiguration.name || !ensMetadataConfiguration.address) {
      throw new error.Error({
        message: "Name or address undefined",
        details: { ensConfiguration },
      });
    }
    try {
      await org.setSingleSigName(
        `${ensConfiguration.name}.radicle.eth`,
        ensMetadataConfiguration.address
      );

      onSubmit();
    } catch (err) {
      buttonsDisabled = false;
      submitButtonCopy = "Link organization to name";

      throw new error.Error({
        message: "Transaction failed",
        source: err,
      });
    }
  }

  async function linkMultiSig() {
    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    if (
      !safeAddress ||
      !ensConfiguration.name ||
      !ensMetadataConfiguration.address
    ) {
      throw new error.Error({
        message: "Name, owner or address undefined",
        details: { ensConfiguration },
      });
    }
    await org.setNameMultisig(
      `${ensConfiguration.name}.radicle.eth`,
      ensMetadataConfiguration.address,
      safeAddress
    );

    onSubmit();
  }
</script>

<div>
  <Emoji emoji="🔗" size="huge" style="margin-bottom: 16px" />
  <HeadlineAndDescription
    headline="Let’s link your name"
    style="margin-bottom: 24px"
    description={`In this last step, we’re updating your organization to point towards your newly created name. Once that’s done, your organization will appear with your new name across Radicle!`} />

  <TextInput
    label="Organization address"
    disabled
    style="margin-bottom: 24px"
    value={ensMetadataConfiguration.address || undefined} />

  <TextInput
    label="Name"
    disabled
    style="margin-bottom: 24px"
    value={`${ensConfiguration.name}.radicle.eth`} />

  <p
    style="color: var(--color-foreground-level-5; margin: 16px 0;"
    class="typo-text-small">
    You can also do this later by selecting "Register ENS Name" and entering
    your existing name.
  </p>

  <ButtonRow
    disableButtons={buttonsDisabled}
    onSubmit={() => {
      safeAddress ? linkMultiSig() : linkSingleSig();
    }}
    canCancel={false}
    confirmCopy={submitButtonCopy} />
</div>
