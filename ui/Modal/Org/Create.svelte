<script lang="typescript">
  import type { Identity } from "../../src/identity";

  import { Copyable, Modal } from "../../DesignSystem/Component";
  import { Avatar, Button, Emoji } from "../../DesignSystem/Primitive";

  import {
    store as walletStore,
    Status as WalletStatus,
  } from "../../src/wallet";
  import * as org from "../../src/org";
  import { ellipsed } from "../../src/style";
  import * as modal from "../../src/modal";

  export let identity: Identity;

  const createOrg = async (owner: string): Promise<void> => {
    modal.hide();
    await org.createOrg(owner, $walletStore.signer, $walletStore.provider);
  };

  $: wallet = $walletStore;

  let walletAddress: string = "";

  $: if ($wallet.status === WalletStatus.Connected) {
    walletAddress = $wallet.connected.account.address;
  } else {
    throw new Error("Org create modal called while wallet is not connected");
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
  .actions {
    display: flex;
    width: 100%;
    gap: 1.5rem;
    justify-content: flex-end;
  }
</style>

<Modal>
  <Emoji emoji={'🎪'} size="huge" style="margin-bottom: 1.5rem;" />
  <h1>Create a new org</h1>
  <p
    style="margin: 0.5rem 0 1.5rem; color: var(--color-foreground-level-6); text-align: center;">
    This will create a gnosis safe that manages the org contract where wallet
    you’ve connected to upstream will be the first member.
  </p>
  <p class="typo-text-bold" style="padding: 0 0 0.5rem 1rem; width: 100%;">
    First member
  </p>
  <div class="member-box">
    <Avatar
      style="margin-right: 16px"
      size="small"
      variant="circle"
      title={identity.metadata.handle}
      avatarFallback={identity.avatarFallback} />
    <Copyable
      showIcon={false}
      styleContent={false}
      copyContent={walletAddress}
      notificationText="Address copied to the clipboard">
      {ellipsed(walletAddress)}
    </Copyable>
  </div>

  <div class="actions">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button on:click={() => createOrg(walletAddress)}>
      Confirm in your wallet
    </Button>
  </div>
</Modal>
