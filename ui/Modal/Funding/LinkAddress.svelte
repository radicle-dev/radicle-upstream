<script lang="typescript">
  import {
    Avatar,
    Button,
    Copyable,
    Emoji,
    Icon,
    Remote,
    TxButton,
  } from "ui/DesignSystem";

  import {
    ClaimsContract,
    claimsAddress,
  } from "../../src/attestation/contract";
  import { lastClaimed } from "../../src/attestation/lastClaimed";
  import * as identity from "../../src/identity";
  import { store as walletStore } from "../../src/wallet";
  import { session } from "ui/src/session";

  import * as modal from "ui/src/modal";

  function onCancel(): void {
    modal.hide();
  }

  $: address = $walletStore.account()?.address || "";

  async function claim(ident: identity.Identity) {
    $lastClaimed = address.toLowerCase();
    modal.hide();
    await identity.claimEthAddress(address);
    const claims = new ClaimsContract(
      $walletStore.signer,
      claimsAddress($walletStore.environment)
    );
    await claims.claim(ident.urn);
  }
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    min-height: 31.25rem;
    text-align: center;
    padding: var(--content-padding);
    width: 37.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
  }

  header {
    padding: 0 var(--content-padding);
  }

  .data {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-evenly;

    width: 100%;
    height: 10.625rem;

    border: 1px solid var(--color-foreground-level-3);
    background-color: var(--color-foreground-level-1);

    margin-top: var(--content-padding);
    padding: calc(var(--content-padding) / 2);
    border-radius: 1rem;
  }

  .radicle-user {
    display: flex;
    align-items: center;
  }
  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: 1.5rem;
  }
</style>

<Remote store={session} let:data={it}>
  <div class="wrapper">
    <Emoji emoji="🧦" size="huge" />

    <header>
      <h1 style="margin-top: 1.5rem;">
        Link your Radicle Identity and Ethereum address
      </h1>
      <p style="margin-top: 1.5rem; padding: 0 4rem" class="typo-text">
        An Ethereum transaction will be sent
      </p>
    </header>

    <div class="data">
      <p class="radicle-user typo-text-bold">
        <Avatar
          size="small"
          avatarFallback={it.identity.avatarFallback}
          variant="circle"
          style="margin-right: 10px" />
        {it.identity.metadata.handle}
      </p>
      <Icon.ChevronUpDown />
      <p class="address typo-text">
        <Copyable
          showIcon={false}
          styleContent={false}
          copyContent={address}
          notificationText="Address copied to the clipboard">
          {address}
        </Copyable>
      </p>
    </div>

    <div class="submit">
      <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
        Cancel
      </Button>

      <TxButton
        dataCy="confirm-button"
        onClick={() => claim(it.identity)}
        style="margin-left: 14px;"
        errorLabel="Failed to claim your Radicle Identity on Ethereum">
        Link your ID
      </TxButton>
    </div>
  </div>
</Remote>
