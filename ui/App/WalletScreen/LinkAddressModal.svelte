<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Avatar, Button, Icon, Identifier } from "ui/DesignSystem";

  import Modal from "ui/App/ModalLayout/Modal.svelte";
  import Remote from "ui/App/Remote.svelte";
  import TransactionButton from "./LinkAddressModal/TransactionButton.svelte";

  import { ClaimsContract, claimsAddress } from "ui/src/attestation/contract";
  import { lastClaimed } from "ui/src/attestation/lastClaimed";
  import { session } from "ui/src/session";
  import { store as walletStore } from "ui/src/wallet";

  import * as identity from "ui/src/identity";
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
</style>

<Remote store={session} let:data={it}>
  <Modal emoji="🧦" title="Link your Radicle ID and Ethereum address">
    <svelte:fragment slot="description">
      An Ethereum transaction will be sent
    </svelte:fragment>

    <div class="data">
      <div style="display: flex;">
        <Avatar
          size="small"
          kind={{ type: "userEmoji", uniqueIdentifier: it.identity.urn }}
          style="margin-right: 0.625rem;" />
        <p class="typo-text">{it.identity.metadata.handle}</p>
      </div>
      <Icon.ChevronUpDown />
      <p class="address typo-text">
        <Identifier value={address} kind="ethAddress" />
      </p>
    </div>

    <svelte:fragment slot="buttons">
      <Button variant="transparent" on:click={onCancel}>Cancel</Button>

      <TransactionButton
        dataCy="confirm-button"
        onClick={() => claim(it.identity)}
        errorLabel="Failed to claim your Radicle ID on Ethereum">
        Link your Radicle ID
      </TransactionButton>
    </svelte:fragment>
  </Modal>
</Remote>
