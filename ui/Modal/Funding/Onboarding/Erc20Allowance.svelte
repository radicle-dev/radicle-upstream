<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Modal, TxButton } from "ui/DesignSystem";

  export let onCancel: () => void;
  export let onConfirm: () => Promise<void>;
</script>

<Modal emoji="🧐" title="One thing before you start">
  <svelte:fragment slot="description">
    Radicle Pools use DAI, an ERC-20 token. To use it, we need you to send a
    signed transaction.
  </svelte:fragment>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" dataCy="cancel" on:click={onCancel}>
      Cancel
    </Button>

    <TxButton
      dataCy="confirm-button"
      onClick={onConfirm}
      errorLabel="Failed to approve ERC-20 for the Radicle Pool">
      Confirm in your wallet
    </TxButton>
  </svelte:fragment>
</Modal>
