<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { ClaimsContract, claimsAddress } from "ui/src/attestation/contract";
  import { lastClaimed } from "ui/src/attestation/lastClaimed";
  import { store as walletStore } from "ui/src/wallet";
  import * as Session from "ui/src/session";
  import * as identity from "ui/src/identity";
  import * as modal from "ui/src/modal";

  import ChevronUpDownIcon from "design-system/icons/ChevronUpDown.svelte";
  import Button from "design-system/Button.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import Modal from "ui/App/ModalLayout/Modal.svelte";
  import TransactionButton from "./LinkAddressModal/TransactionButton.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  function onCancel(): void {
    modal.hide();
  }

  const session = Session.unsealed();

  $: address = $walletStore.getAddress() || "";

  async function claim(ident: identity.Identity): Promise<void> {
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

<Modal emoji="🧦" title="Link your Radicle identity and Ethereum address">
  <svelte:fragment slot="description">
    An Ethereum transaction will be sent
  </svelte:fragment>

  <div class="data">
    <div style="display: flex;">
      <UserIdentity
        urn={session.identity.urn}
        handle={session.identity.metadata.handle} />
    </div>
    <ChevronUpDownIcon />
    <p class="address typo-text">
      <CopyableIdentifier value={address} kind="ethAddress" />
    </p>
  </div>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" on:click={onCancel}>Cancel</Button>

    <TransactionButton
      dataCy="confirm-button"
      onClick={() => claim(session.identity)}
      errorLabel="Failed to claim your Radicle identity on Ethereum">
      Link your Radicle identity
    </TransactionButton>
  </svelte:fragment>
</Modal>
