<script>
  import { link } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import {
    identityAvatarUrlStore,
    identityAvatarFallbackStore,
    identityDisplayNameStore,
    identityHandleStore,
    identityShareableEntityIdentifierStore
  } from "../../store/identity.js";

  import {
    Avatar,
    Button,
    Flex,
    Icon,
    Text,
    Title
  } from "../../DesignSystem/Primitive";
  import { Copyable } from "../../DesignSystem/Component";

  export let onClose;
  let copyIcon = Icon.Copy;

  const afterCopy = () => {
    copyIcon = Icon.Check;
    setTimeout(() => {
      copyIcon = Icon.Copy;
    }, 2000);
  };
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
      <!-- TODO(sarah): actually link to handle registration flow -->
      <a class="registration-link" href={path.registerUser()} use:link>
        register it.
      </a>
    </Text>
    <div class="identity-card">
      <Avatar
        size="huge"
        imageUrl={$identityAvatarUrlStore}
        avatarFallback={$identityAvatarFallbackStore} />
      <div class="identity-card-text-container">
        <Title>{$identityDisplayNameStore || $identityHandleStore}</Title>
        <Copyable {afterCopy}>
          <Flex align="left">
            <Text
              style="color: var(--color-foreground-level-6); white-space:
              nowrap; overflow: hidden; text-overflow: ellipsis; max-width:
              350px;">
              {$identityShareableEntityIdentifierStore}
            </Text>
            <svelte:component this={copyIcon} style="margin-left: 8px;" />
          </Flex>
        </Copyable>
      </div>
    </div>

    <Button on:click={onClose}>Go to profile</Button>
  </div>
</div>
