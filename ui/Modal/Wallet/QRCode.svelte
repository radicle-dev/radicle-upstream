<script lang="typescript">
  import { pop } from "svelte-spa-router";

  import { Illustration, QR, Spinner } from "../../DesignSystem/Component";

  import { uriStore } from "../../src/wallet";
  import { Variant as IllustrationVariant } from "../../src/illustration";

  if ($uriStore === null) pop();

  $: uri = $uriStore!;
</script>

<style>
  .qrcode-modal {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 650px;
    background: var(--color-background);
    border-radius: 0.5rem;

    text-align: center;
  }

  .qrcode-wrapper {
    margin: var(--content-padding) 0;
    padding: calc(var(--content-padding) / 2);

    border: 1px solid var(--color-foreground-level-2);
    border-radius: 16px;

    box-shadow: rgba(0, 0, 0, 0.1) 0px 8px 16px;
  }

  .waiting {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;

    width: 100%;
    margin-top: var(--content-padding);

    padding: calc(var(--content-padding) * 0.75);
    background-color: var(--color-foreground-level-1);
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.25rem;
  }

  .info {
    display: flex;
    flex-direction: column;
    text-align: left;
    margin-left: calc(var(--content-padding) * 0.75);
  }
</style>

<div class="qrcode-modal">
  <Illustration variant={IllustrationVariant.Purse} />

  <h1 style="margin-top: 1.5rem;">Connect your wallet</h1>
  <p style="margin-top: 1.5rem;">
    Scan this QR code with your mobile wallet and follow the instructions.
  </p>

  <div class="qrcode-wrapper">
    <QR size={225} key={uri} />
  </div>

  <p>
    Not working? <a
      href="https://walletconnect.org/wallets"
      class="typo-link typo-text-small-bold">Check if your mobile wallet
      supports WalletConnect</a>.
  </p>

  <div class="waiting">
    <Spinner />
    <div class="info">
      <p class="typo-text-bold">Waiting for you to scan...</p>
      <p style="color: var(--color-foreground-level-5);">
        This won’t cost you any Ether
      </p>
    </div>
  </div>
</div>
