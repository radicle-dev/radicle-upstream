<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import twemoji from "twemoji";

  export let emoji: string;

  type EmojiSize = "small" | "regular" | "large" | "huge";
  export let size: EmojiSize = "regular";

  export let style: string | undefined = undefined;
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

  :global(.emoji.large) {
    height: 2rem;
    width: 2rem;
  }

  :global(.emoji.huge) {
    height: 3rem;
    width: 3rem;
  }
</style>

<div {style} class="container">
  {@html twemoji.parse(emoji, {
    className: `emoji ${size}`,
    base: "/",
    folder: "twemoji",
    ext: ".svg",
  })}
</div>
