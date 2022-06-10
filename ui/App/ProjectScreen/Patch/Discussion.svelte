<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "design-system/TextInput";

  import dayjs from "dayjs";

  import * as Error from "ui/src/error";
  import * as Hotkeys from "ui/src/hotkeys";
  import * as Modal from "ui/src/modal";
  import * as Patch from "ui/src/project/patch";
  import * as Session from "ui/src/session";

  import Avatar from "design-system/Avatar.svelte";
  import Button from "design-system/Button.svelte";
  import Markdown from "design-system/Markdown.svelte";
  import TextArea from "design-system/TextArea.svelte";
  import InfoCircleIcon from "design-system/icons/InfoCircle.svelte";
  import PenIcon from "design-system/icons/Pen.svelte";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import UserIdentity from "ui/App/SharedComponents/UserIdentity.svelte";

  import ManagePeersModal from "../ManagePeersModal.svelte";

  export let projectUrn: string;
  export let patch: Patch.Patch;

  const session = Session.unsealed();

  let comment: string = "";
  let addCommentInProgress = false;
  let preview = false;

  async function addComment() {
    addCommentInProgress = true;
    try {
      await Patch.publishEvent(
        { projectUrn: projectUrn, peerId: patch.peerId, name: patch.id },
        {
          type: "addComment",
          data: {
            comment,
            timestamp: Date.now(),
          },
        }
      );
      comment = "";
    } catch (err: unknown) {
      Error.showNotification(
        new Error.Error({
          message: "Failed to add comment",
          details: {
            projectUrn: projectUrn,
            peerId: patch.peerId,
            name: patch.id,
          },
          source: err,
        })
      );
    } finally {
      addCommentInProgress = false;
    }
  }

  function isCommentValid(comment: string): TextInputValidationState {
    const limit = 4000;
    const length = comment.length;
    const overLimit = length - limit;

    if (length > limit) {
      return {
        type: "invalid",
        message: `Whoa Shakespeare, you're ${overLimit} character${
          overLimit === 1 ? "" : "s"
        } over the ${limit} character limit.`,
      };
    }

    return { type: "valid" };
  }

  $: validationState = isCommentValid(comment);
  $: addCommentDisabled =
    comment.trim() === "" ||
    addCommentInProgress ||
    validationState.type === "invalid";
</script>

<style>
  .wrapper {
    display: flex;
    gap: 1rem;
  }

  .comment {
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-3);
    width: 100%;
    margin-bottom: 1rem;
  }

  .header {
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 0.5rem 0.75rem;
  }

  .banner {
    border-radius: 1rem;
    background-color: var(--color-foreground-level-1);
    display: flex;
    align-items: center;
    padding: 0.8rem 1rem;
    gap: 0.5rem;
    margin-bottom: 2rem;
  }
  .buttons {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    margin-bottom: 4rem;
  }
</style>

<div class="banner">
  <InfoCircleIcon />
  Other user's comments will only show up here if you're tracking their Peer ID.
  <div style="margin-left: auto" />
  <Button
    on:click={() => {
      Modal.toggle(ManagePeersModal);
    }}
    variant="outline"
    icon={PenIcon}>Edit remotes</Button>
</div>

{#each patch.comments as comment}
  <div class="wrapper">
    <div style="padding-top: 2px">
      {#if comment.user}
        <UserIdentity
          size="regular"
          modalStyle="top: 1rem; left: 1.5rem;"
          urn={comment.user.urn} />
      {:else}
        <Avatar
          style="align-items: flex-start;"
          kind={{ type: "unknownUser" }} />
      {/if}
    </div>
    <div class="comment">
      <div class="header">
        <span class="typo-text-bold">
          {#if comment.user}
            {comment.user.metadata.handle}
          {:else}
            <CopyableIdentifier
              style="display: inline-block;"
              showIcon={false}
              value={comment.peerId}
              kind="peerId" />
          {/if}
        </span>
        <span style="color: var(--color-foreground-level-6)">commented</span>
        <span style="color: var(--color-foreground-level-5)"
          >{dayjs().to(dayjs(comment.timestamp))}</span>
      </div>
      <div style="padding: 0.5rem 0.75rem;">
        <Markdown content={comment.comment} markedOptions={{ breaks: true }} />
      </div>
    </div>
  </div>
{/each}

<div class="wrapper typo-wrap">
  <div style="padding-top: 2px">
    <UserIdentity
      size="regular"
      modalStyle="top: 1rem; left: 1.5rem;"
      urn={session.identity.urn} />
  </div>

  {#if preview}
    <div class="comment">
      <div class="header">
        <span class="typo-text-bold">{session.identity.metadata.handle}</span>
        <span style="color: var(--color-foreground-level-6)">preview</span>
      </div>
      <div style="padding: 0.5rem 0.75rem;">
        <Markdown content={comment} markedOptions={{ breaks: true }} />
      </div>
    </div>
  {:else}
    <TextArea
      {validationState}
      on:keydown={event => {
        const modifierKey = Hotkeys.isMac ? event.metaKey : event.ctrlKey;
        if (event.key === "Enter" && modifierKey && !addCommentDisabled) {
          addComment();
        }
      }}
      bind:value={comment}
      placeholder="Leave a comment"
      caption={`Markdown supported. Press ${Hotkeys.osModifierKey}↵ to comment.`} />
  {/if}
</div>
<div class="buttons">
  <Button
    disabled={addCommentDisabled}
    variant="transparent"
    on:click={() => {
      preview = !preview;
    }}>{preview ? "Resume editing" : "Preview"}</Button>

  <Button
    disabled={addCommentDisabled}
    on:click={() => {
      addComment();
    }}>Comment</Button>
</div>
