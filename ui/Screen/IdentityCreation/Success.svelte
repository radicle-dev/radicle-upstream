<script>
  import { createEventDispatcher } from "svelte";

  import { store } from "../../src/identity.ts";

  import { Copyable, Remote } from "../../DesignSystem/Component";
  import {
    Avatar,
    Button,
    Flex,
    Text,
    Title,
  } from "../../DesignSystem/Primitive";

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

  .registration-link {
    color: var(--color-secondary);
    cursor: pointer;
  }

  .registration-link:hover {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container">
  <div class="content">
    <Title variant="big" style="text-align: center;">Identity created ✨</Title>
    <Text style="margin: 20px 0; color: var(--color-foreground-level-5);">
      This is your peer-to-peer identity. Even though your radicleID is unique,
      your handle isn't. To get a unique handle, you have to
      <span
        data-cy="register-identity-link"
        class="registration-link"
        on:click={() => dispatch('register')}>
        register it.
      </span>
    </Text>
    <Remote {store} let:data={identity}>
      <div class="identity-card" data-cy="identity-card">
        <Avatar size="big" avatarFallback={identity.avatarFallback} />
        <div class="identity-card-text-container">
          <Title>{identity.metadata.handle}</Title>
          <Copyable iconSize="normal">
            <Flex align="left">
              <Text
                style="color: var(--color-foreground-level-6); white-space:
                nowrap; overflow: hidden; text-overflow: ellipsis; max-width:
                350px;">
                {identity.shareableEntityIdentifier}
              </Text>
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
