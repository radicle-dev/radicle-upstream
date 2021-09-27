<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { unreachable } from "./lib/unreachable";
  import * as radicleAvatar from "radicle-avatar";
  import Emoji from "./Emoji.svelte";

  export let dataCy: string | undefined = undefined;
  export let style: string | undefined = undefined;

  type EmojiKind =
    | { type: "userEmoji"; uniqueIdentifier: string }
    | { type: "orgEmoji"; uniqueIdentifier: string };

  type Kind =
    | EmojiKind
    | { type: "userImage"; url: string }
    | { type: "orgImage"; url: string }
    | { type: "pendingOrg" }
    | { type: "unknownUser" };

  export let kind: Kind;

  export let size: "small" | "regular" | "huge" = "regular";

  // TODO: memoize this because we call it twice for each emoji component.
  function emojiAvatar(kind: EmojiKind): {
    emoji: string;
    backgroundColor: string;
  } {
    switch (kind.type) {
      case "userEmoji": {
        const avatar = radicleAvatar.generate(
          kind.uniqueIdentifier,
          radicleAvatar.Usage.Identity
        );
        return {
          emoji: avatar.emoji,
          backgroundColor: `rgb(${avatar.background.r}, ${avatar.background.g}, ${avatar.background.b});`,
        };
      }
      case "orgEmoji": {
        const avatar = radicleAvatar.generate(
          kind.uniqueIdentifier,
          radicleAvatar.Usage.Any
        );
        return {
          emoji: avatar.emoji,
          backgroundColor: `rgb(${avatar.background.r}, ${avatar.background.g}, ${avatar.background.b});`,
        };
      }
    }
  }
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-foreground-level-6);
  }

  .circle.small {
    width: 24px;
    height: 24px;
    border-radius: 12px;
  }

  .circle.regular {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }

  .circle.huge {
    width: 120px;
    height: 120px;
    border-radius: 60px;
  }

  .square.small {
    width: 24px;
    height: 24px;
    border-radius: 0.25rem;
  }

  .square.regular {
    width: 32px;
    height: 32px;
    border-radius: 0.5rem;
  }

  .square.huge {
    width: 120px;
    height: 120px;
    border-radius: 1.5rem;
  }

  .avatar {
    display: flex;
    justify-content: center;
    align-items: center;
    user-select: none;
    flex-shrink: 0;
  }

  .pulsate {
    opacity: 1;
    animation: pulsate 3.5s ease-out infinite;
  }

  @keyframes pulsate {
    0%,
    100% {
      opacity: 0.4;
    }
    50% {
      opacity: 1;
    }
  }
</style>

<div
  data-cy={dataCy}
  class="container"
  class:small={size === "small"}
  class:regular={size === "regular"}
  class:huge={size === "huge"}
  {style}>
  {#if kind.type === "userImage"}
    <img
      class="avatar circle"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      src={kind.url}
      alt="user-avatar" />
  {:else if kind.type === "orgImage"}
    <img
      class="avatar square"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      src={kind.url}
      alt="user-avatar" />
  {:else if kind.type === "userEmoji"}
    <div
      class="avatar circle"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      style={`background-color: ${emojiAvatar(kind).backgroundColor}`}
      data-cy="emoji">
      <Emoji {size} emoji={emojiAvatar(kind).emoji} />
    </div>
  {:else if kind.type === "orgEmoji"}
    <div
      class="avatar square"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      style={`background-color: ${emojiAvatar(kind).backgroundColor}`}
      data-cy="emoji">
      <Emoji {size} emoji={emojiAvatar(kind).emoji} />
    </div>
  {:else if kind.type === "pendingOrg"}
    <div
      class="avatar pulsate square"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      style="background-color: var(--color-foreground-level-3);"
      data-cy="emoji" />
  {:else if kind.type === "unknownUser"}
    <div
      class="avatar circle"
      class:small={size === "small"}
      class:regular={size === "regular"}
      class:huge={size === "huge"}
      style="background: var(--color-foreground-level-3)" />
  {:else}
    {unreachable(kind)}
  {/if}
</div>
