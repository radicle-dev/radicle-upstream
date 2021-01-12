<script lang="typescript">
  import { Avatar, Button, Emoji, Icon } from "../../../DesignSystem/Primitive";
  import { Copyable } from "../../../DesignSystem/Component";

  import type { Identity } from "../../../src/identity";
  import type { Address } from "../../../src/funding/pool";

  export let address: Address;
  export let identity: Identity;

  export let onCancel: () => void;
  export let onConfirm: () => void;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
  }

  header {
    padding: 0px var(--content-padding);
  }

  .from-to {
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
    margin-top: 1.5rem;
  }
</style>

<div class="wrapper">
  <Emoji emoji="🧦" size="huge" />

  <header>
    <h1 style="margin-top: 1.5rem;">Add your Ethereum account to Radicle</h1>
    <p
      style="margin-top: 1.5rem; padding: 0 var(--content-padding)"
      class="typo-text">
      Are you sure you want to add your Ethereum address to your Radicle
      account?
    </p>
  </header>

  <div class="from-to">
    <div class="entity">
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
    <Icon.ArrowDown />
    <div class="entity">
      <p class="radicle-user typo-text-bold">
        <Avatar
          size="small"
          avatarFallback={identity.avatarFallback}
          variant="circle"
          style="margin-right: 10px" />
        {identity.metadata.handle}
      </p>
    </div>
  </div>

  <div class="submit">
    <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
      Cancel
    </Button>

    <Button
      dataCy="confirm-button"
      on:click={onConfirm}
      style="margin-left: 14px;">
      Confirm
    </Button>
  </div>
</div>
