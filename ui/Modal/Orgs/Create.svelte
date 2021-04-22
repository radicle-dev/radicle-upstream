<script lang="typescript">
  import { push } from "svelte-spa-router";

  import type { Identity } from "../../src/identity";

  import { Copyable, Modal } from "../../DesignSystem/Component";
  import { Avatar, Button, Emoji } from "../../DesignSystem/Primitive";

  import { store as walletStore } from "../../src/wallet";
  import * as org from "../../src/org";
  import * as path from "../../src/path";
  import { ellipsed } from "../../src/style";
  import * as modal from "../../src/modal";

  export let identity: Identity;

  const orgStore = org.store;

  async function createOrg(owner): void {
    const orgAddr = await org.createOrg(
      owner,
      $walletStore.signer,
      $walletStore.provider
    );

    if (orgAddr) {
      push(path.org(orgAddr));
      modal.hide();
    } else {
      console.log("NO ORGADDR");
    }
  }

  $: wallet = $walletStore;
  $: console.log("ORG_STORE STATE: ", $orgStore);
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
      copyContent={$wallet.connected.account.address}
      notificationText="Address copied to the clipboard">
      {ellipsed($wallet.connected.account.address)}
    </Copyable>
  </div>

  <div class="actions">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button on:click={() => createOrg($wallet.connected.account.address)}>
      Confirm in your wallet
    </Button>
  </div>
</Modal>
