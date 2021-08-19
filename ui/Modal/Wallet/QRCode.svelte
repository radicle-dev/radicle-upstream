<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { qrcode } from "pure-svg-code";

  import { Copyable, Modal } from "ui/DesignSystem";

  import { ellipsed } from "ui/src/style";

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
    <a href="https://walletconnect.org/wallets" class="typo-link">
      View compatible wallets.
    </a>
  </p>

  <div class="qrcode-wrapper">
    <div data-cy="qr-code">
      {@html svgString}
    </div>
  </div>

  <p class="typo-text-bold connector">
    <Copyable showIcon={true} styleContent={false} copyContent={uri}>
      <p class="typo-text-small-mono">{ellipsed(uri, 5)}</p>
    </Copyable>
  </p>
</Modal>
