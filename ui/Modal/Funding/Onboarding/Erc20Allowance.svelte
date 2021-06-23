<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Button, Emoji, TxButton } from "ui/DesignSystem";

  export let onCancel: () => void;
  export let onConfirm: () => Promise<void>;
</script>

<style>
  h1 {
    padding: 0 var(--content-padding);
  }

  h1,
  p,
  .submit {
    margin-top: 1.5rem;
  }

  h1,
  p {
    padding: 0 2.5rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
  }
</style>

<Emoji emoji="🧐" size="huge" />
<h1>One thing before you start</h1>
<p>
  Radicle Pools use DAI, an ERC-20 token. To use it, we need you to send a
  signed transaction.
</p>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onCancel}
    style="margin-right: 1rem">
    Cancel
  </Button>

  <TxButton
    dataCy="confirm-button"
    onClick={onConfirm}
    errorLabel="Failed to approve ERC-20 for the Radicle Pool">
    Confirm in your wallet
  </TxButton>
</div>
