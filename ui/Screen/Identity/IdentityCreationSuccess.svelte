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

  import { Avatar, Button, Text, Title } from "../../DesignSystem/Primitive";

  export let onClose;
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
    background-color: var(--color-almostwhite);
    padding: 24px;
    margin-bottom: 34px;
    width: 100%;
    border-radius: 2px;
  }

  .identity-card-text-container {
    margin-left: 16px;
  }

  .registration-link {
    color: var(--color-purple);
  }

  .registration-link:hover {
    color: var(--color-gray);
  }
</style>

<div class="container">
  <div class="content">
    <Title variant="big" style="text-align: center;">Identity created ✨</Title>
    <Text style="margin: 20px 0; color: var(--color-gray);">
      This is your peer-to-peer identity. Even though your radicleID is unique,
      your handle isn't. To get a unique handle, you have to
      <!-- TODO(sarah): actually link to handle registration flow -->
      <a class="registration-link" href={path.profile()} use:link>
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
        <Text style="color: var(--color-darkgray);">
          {$identityShareableEntityIdentifierStore}
        </Text>
      </div>
    </div>

    <Button on:click={onClose}>Go to profile</Button>
  </div>
</div>
