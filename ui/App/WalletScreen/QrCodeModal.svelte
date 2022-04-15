<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { qrcode } from "pure-svg-code";

  import * as format from "design-system/lib/format";

  import Copyable from "ui/App/SharedComponents/Copyable.svelte";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  export let uri: string;

  $: svgString = qrcode({
    content: uri,
    width: 225,
    height: 225,
    color: "black",
    background: "white",
    ecl: "M",
  });
</script>

<style>
  .qrcode-wrapper {
    width: fit-content;
    margin: var(--content-padding) auto;
    padding: calc(var(--content-padding) / 2);
    border-radius: 1rem;
    background-color: white;
    box-shadow: var(--elevation-medium);
  }

  .connector {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-foreground-level-5);
  }
</style>

<Modal emoji="👛" title="Connect your wallet">
  <p style="text-align: center;">
    Scan this code with your wallet. Not working? <br />
    <a class="typo-link" href="https://walletconnect.com/registry?type=wallet"
      >View compatible wallets</a>
  </p>

  <div class="qrcode-wrapper">
    <div data-cy="qr-code">
      {@html svgString}
    </div>
  </div>

  <p class="typo-text-bold connector">
    <Copyable name="WalletConnect link" clipboardContent={uri}>
      <p class="typo-text-small-mono">{format.shorten(uri, 8, 5)}</p>
    </Copyable>
  </p>
</Modal>
