<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Identity } from "ui/src/identity";

  import { Avatar, Button, Copyable, Modal } from "ui/DesignSystem";

  import * as org from "ui/src/org";
  import { ellipsed } from "ui/src/style";
  import * as modal from "ui/src/modal";

  export let identity: Identity;
  export let walletAddress: string;

  const createOrg = async (owner: string): Promise<void> => {
    modal.hide();
    await org.createOrg(owner);
  };
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

<Modal
  emoji="🎪"
  title="Create a new org"
  desc="This will create a gnosis safe that manages the org contract where wallet
you’ve connected to upstream will be the first member.">
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
