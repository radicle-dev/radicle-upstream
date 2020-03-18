<script>
  import Title from "./Title.svelte";

  export let style = null;

  // the hierarchy of usage for the following avatars is:
  // imageUrl > avatarFallback
  export let imageUrl = null;
  export let avatarFallback = null; // {emoji: <emoji>, background: {r: <r>, g: <g>, b: <b>}};
  export let title = null;

  export let variant = "user"; // user | project
  export let size = "regular"; // regular | medium | big

  const fmt = background => {
    return `rgb(${background.r}, ${background.g}, ${background.b})`;
  };

  $: avatarClass = [variant, size].join(" ");
</script>

<style>
  img,
  .user.regular {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }

  .user.medium {
    width: 36px;
    height: 36px;
    border-radius: 18px;
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

  .project.big {
    width: 64px;
    height: 64px;
  }

  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-darkgray);
  }

  .avatar {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

<div class={`container ${size}`} {style}>
  {#if imageUrl}
    <img
      class={avatarClass}
      src={imageUrl}
      alt="user-avatar"
      onerror="this.style.display='none'" />
  {:else if avatarFallback}
    <div
      class={`avatar ${avatarClass}`}
      style="background: {fmt(avatarFallback.background)}">
      <Title variant={size} style="min-width: 27px; text-align: end;">
        {avatarFallback.emoji}
      </Title>
    </div>
  {:else}
    <div
      class={`avatar ${avatarClass}`}
      style="background: var(--color-lightgray)" />
  {/if}

  {#if title && (size === 'regular' || size === 'medium')}
    <Title style="white-space: nowrap; margin-left: 16px">{title}</Title>
  {/if}

  {#if title && size === 'big'}
    <Title variant="big" style="white-space: nowrap; margin-left: 16px">
      {title}
    </Title>
  {/if}
</div>
