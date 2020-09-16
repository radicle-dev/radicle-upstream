<script lang="ts">
  import type { EmojiAvatar, RGBValue } from "../../src/avatar";

  import Emoji from "./Emoji.svelte";
  import Icon from "./Icon";

  export let style = "";
  export let dataCy = "";

  // the hierarchy of usage for the following avatars is:
  // imageUrl > avatarFallback
  export let imageUrl = "";
  export let avatarFallback: EmojiAvatar | undefined = undefined; // {emoji: <emoji>, background: {r: <r>, g: <g>, b: <b>}};
  export let title = "";
  export let registered = false;

  export let variant: "circle" | "square" = "circle";

  type AvatarSize = "small" | "regular" | "medium" | "big" | "huge";
  export let size: AvatarSize = "regular";

  const fmt = (background: RGBValue) =>
    `rgb(${background.r}, ${background.g}, ${background.b})`;

  $: avatarClass = [variant, size].join(" ");
</script>

<style>
  .circle.small {
    width: 24px;
    height: 24px;
    border-radius: 16px;
  }

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
    width: 72px;
    height: 72px;
    border-radius: 36px;
  }

  .circle.huge {
    width: 120px;
    height: 120px;
    border-radius: 60px;
  }

  .avatar.circle.big {
    line-height: 68px;
  }

  .square {
    border-radius: 4px;
  }

  .square.small {
    width: 24px;
    height: 24px;
    border-radius: 2px;
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
    width: 72px;
    height: 72px;
    border-radius: 4px;
  }

  .square.huge {
    width: 120px;
    height: 120px;
    border-radius: 8px;
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
    flex-shrink: 0;
  }

  .image {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }
</style>

<div data-cy={dataCy} class={`container ${size}`} {style}>
  {#if imageUrl}
    <img class={`image ${avatarClass}`} src={imageUrl} alt="user-avatar" />
  {:else if avatarFallback}
    <div
      class={`avatar ${avatarClass}`}
      style="background: {fmt(avatarFallback.background)}"
      data-cy="emoji">
      <Emoji {size} emoji={avatarFallback.emoji} />
    </div>
  {:else}
    <div
      class={`avatar ${avatarClass}`}
      style="background: var(--color-foreground-level-3)" />
  {/if}

  {#if title}
    {#if size === 'big' || size === 'huge'}
      <h2 style="white-space: nowrap; margin-left: 12px">{title}</h2>
    {:else if size === 'small'}
      <p
        class="typo-text-bold"
        style="white-space: nowrap; margin-left: 0.5rem; color:
        var(--title-color, var(--color-foreground))">
        {title}
      </p>
      {#if registered}
        <Icon.RegisteredSmall
          dataCy="registered-badge"
          style="fill: var(--color-primary);" />
      {/if}
    {:else}
      <p
        class="typo-text-bold"
        style="white-space: nowrap; margin-left: 12px; color: var(--title-color,
        var(--color-foreground))">
        {title}
      </p>
      {#if registered}
        <Icon.RegisteredSmall
          dataCy="registered-badge"
          style="fill: var(--color-primary);" />
      {/if}
    {/if}
  {/if}
</div>
