<script lang="typescript">
  import { Copyable, Emoji, Modal, QR } from "ui/DesignSystem";

  import { uriStore } from "ui/src/wallet";
  import { ellipsed } from "ui/src/style";

  $: uri = $uriStore || "";
</script>

<style>
  .container {
    text-align: center;
  }
  .qrcode-wrapper {
    width: fit-content;
    margin: var(--content-padding) auto;
    padding: calc(var(--content-padding) / 2);

    border: 1px solid var(--color-foreground-level-2);
    border-radius: 1rem;

    background-color: white;
    box-shadow: rgba(0, 0, 0, 0.1) 0px 0.5rem 1rem;
  }

  .connector {
    display: flex;
    align-items: center;
    justify-content: center;

    padding-top: 0.625rem;
    color: var(--color-foreground-level-5);
  }
</style>

<Modal>
  <div class="container">
    <Emoji emoji="👛" size="huge" />
    <h1 style="margin-top: 1.5rem;">Connect your wallet</h1>
    <p style="margin-top: 1.5rem;">
      Scan this code with your wallet. Not working? <br />
      <a href="https://walletconnect.org/wallets" class="typo-link">
        View compatible wallets.
      </a>
    </p>

    <div class="qrcode-wrapper">
      <QR key={uri} />
      <p class="typo-text-bold connector">
        <Copyable showIcon={true} styleContent={false} copyContent={uri}>
          <p class="typo-text-small-mono">{ellipsed(uri, 5)}</p>
        </Copyable>
      </p>
    </div>
  </div>
</Modal>
