<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onMount } from "svelte";

  import Emoji from "ui/DesignSystem/Emoji.svelte";
  import TextInput from "ui/DesignSystem/TextInput.svelte";
  import { setName } from "ui/src/org";
  import type { Registration } from "ui/src/org/ensResolver";

  import type { EnsConfiguration, EnsMetadataPayload } from "../ens-flow.types";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

  export let onSubmit: () => void = () => {};
  export let registration: Registration | undefined = undefined;
  export let ensConfiguration: EnsConfiguration;
  export let ensMetadataConfiguration: EnsMetadataPayload;

  let buttonsDisabled = false;
  let submitButtonCopy = "Link organization to name";

  onMount(() => {
    /*
    There's already a registration for the org, and that
    registration has the same name as that entered in the name
    entry step, so we can skip linking.
    */
    if (
      registration &&
      registration.name === `${ensConfiguration.name}.radicle.eth`
    ) {
      onSubmit();
    }
  });

  // TODO: Implement this for a multisig org
  async function handleSubmit() {
    buttonsDisabled = true;
    submitButtonCopy = "Waiting for transaction confirmation...";

    if (!ensConfiguration.name || !ensMetadataConfiguration.address) {
      throw new Error("Name or address undefined");
    }
    try {
      await setName(
        `${ensConfiguration.name}.radicle.eth`,
        ensMetadataConfiguration.address
      );

      onSubmit();
    } catch (e) {
      buttonsDisabled = false;
      submitButtonCopy = "Link organization to name";

      throw e;
    }
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
    onSubmit={handleSubmit}
    canCancel={false}
    confirmCopy={submitButtonCopy} />
</div>
