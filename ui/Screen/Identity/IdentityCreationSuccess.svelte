<script>
  import { getClient, query } from "svelte-apollo";
  import { gql } from "apollo-boost";
  import { link } from "svelte-spa-router";

  import * as path from "../../lib/path.js";
  import {
    avatarUrlStore,
    displayNameStore,
    handleStore,
    shareableEntityIdentifierStore
  } from "../../store/identity.js";

  import { Avatar, Button, Text, Title } from "../../DesignSystem/Primitive";

  export let onClose,
    onError = null;

  const client = getClient();
  let avatarFallback = null;

  const getAvatarUrl = async () => {
    // no need to make this request if we've already got a URL
    if ($avatarUrlStore) return;

    try {
      const GET_AVATAR = gql`
        query Query($handle: ID!) {
          avatar(handle: $handle) {
            emoji
            background {
              r
              g
              b
            }
          }
        }
      `;

      const avatarFallbackResponse = await query(client, {
        query: GET_AVATAR,
        variables: { handle: $handleStore }
      }).result();

      avatarFallback = avatarFallbackResponse.data.avatar;
    } catch (error) {
      onError(error);
    }
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
      <a class="registration-link" href={path.projects()} use:link>
        register it.
      </a>
    </Text>
    <div class="identity-card">
      {#await getAvatarUrl() then result}
        <Avatar size="huge" imageUrl={$avatarUrlStore} {avatarFallback} />
      {/await}
      <div class="identity-card-text-container">
        <Title>{$displayNameStore || $handleStore}</Title>
        <Text style="color: var(--color-darkgray);">
          {$shareableEntityIdentifierStore}
        </Text>
      </div>
    </div>

    <Button on:click={onClose}>Go to profile</Button>
  </div>
</div>
