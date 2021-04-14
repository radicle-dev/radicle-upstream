<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import { Copyable, Modal } from "../../DesignSystem/Component";
  import { Avatar, Button, Emoji } from "../../DesignSystem/Primitive";
  import { store, Status } from "../../src/wallet";
  import * as org from "../../src/org";
  import { ellipsed } from "../../src/style";
  // import type { UnsealedSession } from "../../src/session";

  const dispatch = createEventDispatcher();
  const orgStore = org.store;

  function createOrg(owner): void {
    org.createOrg(owner, $store.signer);
  }

  $: wallet = $store;

  // const session = getContext();
  // console.log(session)
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

{#if $wallet.status === Status.Connected}
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
        title="cloudhead" />
      <!-- title={session.identity.metadata.handle}
      avatarFallback={session.identity.avatarFallback} /> -->
      <Copyable
        showIcon={false}
        styleContent={false}
        copyContent={$wallet.connected.account.address}
        notificationText="Address copied to the clipboard">
        {ellipsed($wallet.connected.account.address)}
      </Copyable>
    </div>
    <!-- <p>Status: {$orgStore}</p> -->
    <div class="actions">
      <Button variant="transparent" on:click={() => dispatch('hide')}>
        Cancel
      </Button>
      <Button on:click={() => createOrg($wallet.connected.account.address)}>
        Create ({$orgStore})
      </Button>
    </div>
  </Modal>
{:else}
  <p>Not connected</p>
{/if}
