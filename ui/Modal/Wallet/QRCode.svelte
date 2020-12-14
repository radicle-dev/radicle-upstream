<script lang="typescript">
  import { pop } from "svelte-spa-router";

  import { Copyable, Illustration, QR } from "../../DesignSystem/Component";

  import { uriStore } from "../../src/wallet";
  import { Variant as IllustrationVariant } from "../../src/illustration";
  import { ellipsed } from "../../src/style";

  $: $uriStore || pop();
  $: uri = $uriStore || "";
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

  .connector {
    display: flex;
    align-items: center;
    justify-content: center;

    padding-top: 10px;
    color: var(--color-foreground-level-5);
  }
</style>

<div class="qrcode-modal">
  <Illustration variant={IllustrationVariant.Purse} />

  <h1 style="margin-top: 1.5rem;">Connect your wallet</h1>
  <p style="margin-top: 1.5rem;">
    Scan this code with your wallet. Not working?
    <a href="https://walletconnect.org/wallets" class="typo-link">
      View compatible wallets.
    </a>
  </p>

  <div class="qrcode-wrapper">
    <QR key={uri} />
    <p class="typo-text-bold connector">
      <Copyable showIcon={true} styleContent={false} copyContent={uri}>
        <p
          class="typo-text-small-mono"
          style="color: var(--color-foreground-level-6)">
          {ellipsed(uri, 5)}
        </p>
      </Copyable>
    </p>
  </div>
</div>
