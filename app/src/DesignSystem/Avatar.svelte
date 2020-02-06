<script>
  import { hash } from "../hash.js";
  import Title from "./Title.svelte"

  export let style = null;

  export let imageUrl = `https://avatars.dicebear.com/v2/avataaars/S7oswrhcNJkjzUhNW33S.svg`;
  export let title = null

  export let variant = "user"; // user | project
  export let size = "regular"; // regular | big

  $: avatarClass = [variant, size].join(" ");
</script>

<style>
  img,
  .user.regular {
    width: 34px;
    height: 34px;
    border-radius: 17px;
  }

  .user.big {
    width: 68px;
    height: 68px;
    border-radius: 34px;
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

  img {
    margin-right: 16px;
  }

  div {
    display: flex;
    align-items: center;
  }

  div.regular {
    color: var(--color-darkgray);
  }

  div.big {
    white-space: nowrap;
  }
</style>

<div class={size} {style}>
  <img class={avatarClass} src={imageUrl} alt="user-avatar" />
  {#if title}
    <Title size={size}>
      {title}
    </Title>
  {/if}
</div>
