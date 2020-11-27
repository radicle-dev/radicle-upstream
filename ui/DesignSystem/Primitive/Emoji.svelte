<script lang="typescript">
  import twemoji from "twemoji";

  export let emoji: string;

  type EmojiSize = "small" | "regular" | "medium" | "big" | "huge";
  export let size: EmojiSize = "regular";

  export let style: string = "";
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.emoji.small) {
    height: 0.75rem;
    width: 0.75rem;
  }

  :global(.emoji.regular) {
    height: 1rem;
    width: 1rem;
  }

  :global(.emoji.medium) {
    height: 1.125rem;
    width: 1.125rem;
  }

  :global(.emoji.big) {
    height: 2.25rem;
    width: 2.25rem;
  }

  :global(.emoji.huge) {
    height: 3rem;
    width: 3rem;
  }
</style>

<div {style} class="container">
  {@html twemoji.parse(emoji, {
    className: `emoji ${size}`,
    base: '',
    folder: 'twemoji/',
    ext: '.svg',
  })}
</div>
