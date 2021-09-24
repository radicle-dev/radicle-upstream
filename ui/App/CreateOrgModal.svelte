<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Identity } from "ui/src/identity";

  import {
    Avatar,
    Button,
    CopyableIdentifier,
    RadioOption,
    TextInput,
  } from "ui/DesignSystem";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  import * as ethereum from "ui/src/ethereum";
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";
  import * as validation from "ui/src/validation";

  export let identity: Identity;
  export let walletAddress: string;

  let ownerAddress: string = walletAddress;

  let isMultiSig: boolean | undefined = undefined;
  let ownerValidationState: validation.ValidationState;
  let validOwnerAddress = false;

  $: {
    if (ownerAddress.match(ethereum.VALID_ADDRESS_MATCH)) {
      ownerValidationState = {
        status: validation.ValidationStatus.Success,
      };
      validOwnerAddress = true;
    } else {
      ownerValidationState = {
        status: validation.ValidationStatus.Error,
        message: "This does not look like a valid Ethereum address",
      };
      validOwnerAddress = false;
    }
  }

  async function createOrg(): Promise<void> {
    modal.hide();
    if (isMultiSig) {
      await org.createOrg(walletAddress, true);
    } else {
      await org.createOrg(ownerAddress, false);
    }
  }
</script>

<style>
  .member-box {
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: 1rem;
    margin-bottom: 1.5rem;
    border: 1px solid var(--color-foreground-level-2);
    border-radius: 0.5rem;
    color: var(--color-foreground-level-6);
  }
</style>

<Modal emoji="🎪" title="Create a new org">
  <svelte:fragment slot="description">
    This will create a Gnosis Safe that manages the org contract where the
    wallet you’ve connected to upstream will be the first member.
  </svelte:fragment>

  <RadioOption
    active={isMultiSig !== undefined && !isMultiSig}
    on:click={ev => {
      ev.stopPropagation();
      isMultiSig = false;
    }}>
    <div slot="option-header" style="margin: 1rem;">
      <p
        class="typo-text-bold"
        style="margin-bottom: 1rem; color: var(--color-foreground-level-6); text-align: center;">
        Single-signer
      </p>
      <p style="color: var(--color-foreground-level-6); text-align: center">
        Creates an org with the specified address as the only owner. Org
        transactions such as anchoring can be signed and executed directly from
        your wallet.
      </p>
    </div>
    <div slot="option-body">
      <p
        class="typo-text-bold"
        style="text-align: left; padding: 0 0 0.5rem 1rem; width: 100%;">
        Owner
      </p>
      <TextInput
        bind:value={ownerAddress}
        placeholder="Enter owner address"
        showSuccessCheck
        validation={ownerValidationState} />
    </div>
  </RadioOption>

  <RadioOption
    active={isMultiSig !== undefined && isMultiSig}
    on:click={ev => {
      ev.stopPropagation();
      isMultiSig = true;
    }}>
    <div slot="option-header" style="margin: 1rem;">
      <p
        class="typo-text-bold"
        style="margin-bottom: 1rem; color: var(--color-foreground-level-6); text-align: center;">
        Multi-signer
      </p>
      <p style="color: var(--color-foreground-level-6); text-align: center">
        Creates an org with a Gnosis Safe as its owner, and the specified
        address as its first member. Transactions such as anchoring have to be
        approved by a quorum of signers.
      </p>
    </div>
    <div slot="option-body">
      <p
        class="typo-text-bold"
        style="text-align: left; padding: 0 0 0.5rem 1rem; width: 100%;">
        First member
      </p>
      <div class="member-box">
        <div style="display: flex;">
          <Avatar
            style="margin-right: 0.625rem;"
            size="small"
            kind={{ type: "userEmoji", uniqueIdentifier: identity.urn }} />
          <p class="typo-text">{identity.metadata.handle}</p>
        </div>
        <CopyableIdentifier value={walletAddress} kind="ethAddress" />
      </div>
    </div>
  </RadioOption>

  <svelte:fragment slot="buttons">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button
      disabled={isMultiSig === undefined || (!isMultiSig && !validOwnerAddress)}
      on:click={() => createOrg()}>
      Confirm in your wallet
    </Button>
  </svelte:fragment>
</Modal>
