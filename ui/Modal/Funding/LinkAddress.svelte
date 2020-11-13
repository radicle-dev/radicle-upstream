<script lang="typescript">
  import { Illustration, Copyable } from "../../DesignSystem/Component";
  import { Avatar, Icon, Button } from "../../DesignSystem/Primitive";

  import { wallet } from "../../src/wallet";
  import { session } from "../../src/session";
  import * as identity from "../../src/identity";

  import { Variant as IllustrationVariant } from "../../src/illustration";
  import * as modal from "../../src/modal";

  async function onConfirmed(): Promise<void> {
    return identity
      .linkEthereumAddress($wallet.connected.account.address)
      .then(() => modal.hide());
  }
</script>

<style>
  .qrcode-modal {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    padding: var(--content-padding);
    width: 600px;
    background: var(--color-background);
    border-radius: 0.5rem;

    text-align: center;
  }

  header {
    padding: 0px var(--content-padding);
  }

  .data {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-evenly;

    width: 100%;
    height: 170px;

    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);

    margin-top: var(--content-padding);
    padding: calc(var(--content-padding) / 2);
    border-radius: 16px;
  }

  .radicle-user {
    display: flex;
    align-items: center;
  }
  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<div class="qrcode-modal">
  <Illustration variant={IllustrationVariant.Socks} />

  <header>
    <h1 style="margin-top: 1.5rem;">Link your Ethereum Account</h1>
    <p style="margin-top: 1.5rem;" class="typo-text">
      Are you sure you want to add your ethereum address to your Radicle
      account?
    </p>
  </header>

  <div class="data">
    <p class="address typo-text">
      <Copyable
        showIcon={false}
        styleContent={false}
        copyContent={$wallet.connected.account.address}
        notificationText="Address copied to the clipboard">
        {$wallet.connected.account.address}
      </Copyable>
    </p>
    <Icon.Link />
    <p class="radicle-user typo-text-bold">
      <Avatar
        size="small"
        avatarFallback={$session.data.identity.avatarFallback}
        variant="circle"
        style="margin-right: 10px" />
      {$session.data.identity.metadata.handle}
    </p>
  </div>

  <div class="submit">
    <Button
      variant="transparent"
      dataCy="cancel-topup"
      on:click={() => modal.hide()}>
      Cancel
    </Button>

    <Button dataCy="confirm-button" on:click={onConfirmed}>Confirm</Button>
  </div>
</div>
