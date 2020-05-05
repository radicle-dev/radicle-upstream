<script>
  import Title from "./Title.svelte";

  export let style = null;

  // the hierarchy of usage for the following avatars is:
  // imageUrl > avatarFallback
  export let imageUrl = null;
  export let avatarFallback = null; // {emoji: <emoji>, background: {r: <r>, g: <g>, b: <b>}};
  export let title = null;

  export let variant = "circle"; // circle | square
  export let size = "regular"; // small | regular | medium | big | huge

  const fmt = background => {
    return `rgb(${background.r}, ${background.g}, ${background.b})`;
  };

  $: avatarClass = [variant, size].join(" ");
</script>

<style>
  .circle.small {
    width: 24px;
    height: 24px;
    border-radius: 16px;
  }

  img,
  .circle.regular {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }

  .circle.small {
    width: 24px;
    height: 24px;
    border-radius: 12px;
  }

  .circle.medium {
    width: 36px;
    height: 36px;
    border-radius: 18px;
  }

  .circle.big {
    width: 68px;
    height: 68px;
    border-radius: 34px;
  }

  .circle.huge {
    width: 72px;
    height: 72px;
    border-radius: 36px;
  }

  .avatar.circle.big {
    line-height: 68px;
  }

  .square {
    border-radius: 2px;
  }

  .square.small {
    width: 24px;
    height: 24px;
  }

  .square.regular {
    width: 32px;
    height: 32px;
  }

  .square.medium {
    width: 36px;
    height: 36px;
  }

  .square.big {
    width: 64px;
    height: 64px;
  }

  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-foreground-level-6);
  }

  .avatar {
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;
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
      <Title variant={size} style="min-width: 27px; text-align: center;">
        {avatarFallback.emoji}
      </Title>
    </div>
  {:else}
    <div
      class={`avatar ${avatarClass}`}
      style="background: var(--color-foreground-level-3)" />
  {/if}

  {#if title}
    {#if size === 'big'}
      <Title variant="big" style="white-space: nowrap; margin-left: 12px">
        {title}
      </Title>
    {:else if size === 'small'}
      <Title
        style="white-space: nowrap; margin-left: 0.5rem; color:
        var(--title-color, var(--color-foreground))">
        {title}
      </Title>
    {:else}
      <Title
        style="white-space: nowrap; margin-left: 12px; color: var(--title-color,
        var(--color-foreground))">
        {title}
      </Title>
    {/if}
  {/if}
</div>
