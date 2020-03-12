<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  import Title from "./Title.svelte";

  export let style = null;

  // the hierarchy of usage for the following avatars is:
  // imageUrl > avatarFallback > handle
  export let imageUrl = null;
  export let avatarFallback = null; // {emoji: <emoji>, background: {r: <r>, g: <g>, b: <b>}};
  export let handle = null; // handle for querying the fallback avatar
  export let title = null;

  export let variant = "user"; // user | project
  export let size = "regular"; // regular | medium | big

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

  const client = getClient();

  let avatar = null;
  if (handle) {
    avatar = query(client, {
      query: GET_AVATAR,
      variables: { handle: handle }
    });
  }

  const fmt = background => {
    return `rgb(${background.r}, ${background.g}, ${background.b})`;
  };

  $: avatarClass = [variant, size].join(" ");
</script>

<style>
  img,
  .user.regular {
    width: 34px;
    height: 34px;
    border-radius: 17px;
  }

  .avatar.user.regular {
    line-height: 34px;
  }

  .user.medium {
    width: 36px;
    height: 36px;
    border-radius: 18px;
  }

  .avatar.user.medium {
    line-height: 36px;
  }

  .user.big {
    width: 68px;
    height: 68px;
    border-radius: 34px;
  }

  .avatar.user.big {
    line-height: 68px;
  }

  .project {
    border-radius: 2px;
  }

  .project.regular {
    width: 32px;
    height: 32px;
  }

  .avatar.project.regular {
    line-height: 32px;
  }

  .project.big {
    width: 64px;
    height: 64px;
  }

  .avatar.project.big {
    line-height: 64px;
  }

  .container {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .avatar {
    text-align: center;
  }
</style>

<div class={`container ${size}`} {style}>
  {#if imageUrl}
    <img class={avatarClass} src={imageUrl} alt="user-avatar" />
  {:else if avatarFallback}
    <div
      class={`avatar ${avatarClass}`}
      style="background: {fmt(avatarFallback.background)}">
      <Title variant={size} style="line-height: inherit">
        {avatarFallback.emoji}
      </Title>
    </div>
  {:else if avatar}
    {#await $avatar then result}
      <div
        class={`avatar ${avatarClass}`}
        style="background: {fmt(result.data.avatar.background)}">
        <Title variant={size} style="line-height: inherit">
          {result.data.avatar.emoji}
        </Title>
      </div>
    {/await}
  {:else}
    <!-- TODO: Remove when all avatars use the new fallback data or add placeholder -->
    <img
      class={avatarClass}
      src="https://avatars.dicebear.com/v2/avataaars/S7oswrhcNJkjzUhNW33S.svg"
      alt="user-avatar" />
  {/if}

  {#if title && size === 'regular'}
    <Title
      style="color: var(--color-darkgray); white-space: nowrap; margin-left:
      16px">
      {title}
    </Title>
  {/if}

  {#if title && size === 'big'}
    <Title variant="big" style="white-space: nowrap; margin-left: 16px">
      {title}
    </Title>
  {/if}
</div>
