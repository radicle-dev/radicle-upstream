<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Modal } from "ui/DesignSystem";

  import Receivers from "ui/DesignSystem/Funding/Pool/Receivers.svelte";
  import type { Receivers as PoolReceivers } from "ui/src/funding/pool";

  export let receivers: PoolReceivers;
  export let onBack: () => void;
  export let onContinue: () => void;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      onContinue();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />

<Modal emoji="💸" title="Add receivers">
  <svelte:fragment slot="description">
    Add receivers to your outgoing support by clicking the “Support” button on
    profiles or project pages.
  </svelte:fragment>

  <Receivers bind:receivers editing={true} alignment="center" />
  <p
    class="typo-text-small"
    style="margin-top: 1.5rem; color: var(--color-foreground-level-5)">
    Tip: You can also add new receivers later on by clicking on “Support” on
    their profile.
  </p>
  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="cancel" on:click={onBack}>
      Back
    </Button>

    <Button dataCy="confirm-button" on:click={onContinue}>Continue</Button>
  </svelte:fragment>
</Modal>
