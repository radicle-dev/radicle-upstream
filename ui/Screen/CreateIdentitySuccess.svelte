<script>
  import { replace } from "svelte-spa-router";

  import { currentIdentityStore } from "../store/identity.js";

  import { Button, Text, Title } from "../DesignSystem/Primitive";
  import { Avatar, ModalLayout } from "../DesignSystem/Component";

  const onClose = () => replace("/projects");
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

<ModalLayout {onClose}>
  <div class="container">
    <div class="content">
      <Title variant="big" style="text-align: center;">
        Identity created ✨
      </Title>
      <Text style="margin: 20px 0; color: var(--color-gray);">
        This is your peer-to-peer identity. Even though your radicleID is
        unique, your handle isn't. To get a unique handle, you have to
        <!-- TODO: actually link to handle registration flow -->
        <a class="registration-link" href="#">register it.</a>
      </Text>
      <div class="identity-card">
        <Avatar size="xl" />
        <div class="identity-card-text-container">
          {#if $currentIdentityStore && $currentIdentityStore.displayName}
            <Title>{$currentIdentityStore.displayName}</Title>
          {/if}
          <Text style="color: var(--color-darkgray);">
            {$currentIdentityStore && $currentIdentityStore.shareableEntityIdentifier}
          </Text>
        </div>
      </div>

      <Button on:click={onClose}>Go to profile</Button>
    </div>
  </div>
</ModalLayout>
