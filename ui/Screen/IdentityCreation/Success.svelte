<script>
  import { createEventDispatcher } from "svelte";

  import { store } from "../../src/identity.ts";

  import { Copyable, Remote } from "../../DesignSystem/Component";
  import { Avatar, Button, Flex } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .identity-card {
    display: flex;
    align-items: center;
    background-color: var(--color-foreground-level-1);
    padding: 24px;
    margin-bottom: 34px;
    width: 100%;
    border-radius: 2px;
  }

  .identity-card-text-container {
    margin-left: 16px;
  }
</style>

<div class="container">
  <div class="content">
    <h2 style="text-align: center;">Identity created ✨</h2>
    <p style="margin: 20px 0; color: var(--color-foreground-level-5);">
      This is your peer-to-peer identity.
    </p>
    <Remote {store} let:data={identity}>
      <div class="identity-card" data-cy="identity-card">
        <Avatar size="big" avatarFallback={identity.avatarFallback} />
        <div class="identity-card-text-container">
          <p class="typo-text-bold">{identity.metadata.handle}</p>
          <Copyable>
            <Flex align="left">
              <p
                class="typo-overflow-ellipsis"
                style="color: var(--color-foreground-level-6); max-width: 350px;">
                {identity.shareableEntityIdentifier}
              </p>
            </Flex>
          </Copyable>
        </div>
      </div>
    </Remote>

    <Button dataCy="go-to-profile-button" on:click={() => dispatch('close')}>
      Go to profile
    </Button>
  </div>
</div>
