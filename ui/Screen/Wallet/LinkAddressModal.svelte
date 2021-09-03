<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import {
    Avatar,
    Button,
    Icon,
    Identifier,
    Modal,
    Remote,
    TxButton,
  } from "ui/DesignSystem";

  import { ClaimsContract, claimsAddress } from "ui/src/attestation/contract";
  import { lastClaimed } from "ui/src/attestation/lastClaimed";
  import * as identity from "ui/src/identity";
  import { store as walletStore } from "ui/src/wallet";
  import { session } from "ui/src/session";

  import * as modal from "ui/src/modal";

  function onCancel(): void {
    modal.hide();
  }

  $: address = $walletStore.getAddress() || "";

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
</style>

<Remote store={session} let:data={it}>
  <Modal emoji="🧦" title="Link your Radicle Identity and Ethereum address">
    <svelte:fragment slot="description">
      An Ethereum transaction will be sent
    </svelte:fragment>

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
        <Identifier value={address} kind="ethAddress" />
      </p>
    </div>

    <svelte:fragment slot="buttons">
      <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
        Cancel
      </Button>

      <TxButton
        dataCy="confirm-button"
        onClick={() => claim(it.identity)}
        errorLabel="Failed to claim your Radicle Identity on Ethereum">
        Link your ID
      </TxButton>
    </svelte:fragment>
  </Modal>
</Remote>
