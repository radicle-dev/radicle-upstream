<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Modal } from "ui/DesignSystem";

  export let onCancel: () => void;
  export let onContinue: () => void;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      onContinue();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />

<Modal emoji="💸" title="Stream digital money to all your favorites">
  <svelte:fragment slot="description">
    Streaming sets up an Ethereum smart contract that sends money out certain
    speed, defined by you, to a group of people or projects.
  </svelte:fragment>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="cancel" on:click={onCancel}>
      Cancel
    </Button>

    <Button dataCy="confirm-button" on:click={onContinue}>Continue</Button>
  </svelte:fragment>
</Modal>
