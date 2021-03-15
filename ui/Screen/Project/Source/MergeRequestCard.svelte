<script lang="ts">
  import { getContext } from "svelte";

  import { Avatar, Icon } from "../../../DesignSystem/Primitive";
  import CheckoutMergeRequestButton from "./CheckoutMergeRequestButton.svelte";
  import AcceptMergeRequestButton from "./AcceptMergeRequestButton.svelte";

  import type { UnsealedSession } from "../src/session";
  import type { MergeRequest } from "../../../src/source";
  import { isMaintainer } from "../../../src/project";

  export let mergeRequest: MergeRequest;

  const session = getContext("session") as UnsealedSession;

  const mergeInfo = mergeRequest && mergeRequest.merged ? "Closed" : "Opened";
  const showCheckout = isMaintainer(
    session.identity.urn,
    getContext("project-page").project
  );
</script>

<style>
  .merge-request-card {
    display: flex;
    max-height: 3.2rem;
    justify-content: space-between;
    width: 100%;
  }

  .left {
    display: flex;
    flex-direction: row;
  }

  .info-column {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
    margin-right: 1.5rem;
  }

  .title-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
  }

  .title-row p {
    color: var(--color-foreground-level-6);
  }

  .desc-row {
    color: var(--color-foreground-level-6);
    display: flex;
    margin-top: 0.125rem;
    width: -webkit-fill-available;
  }

  .actions-column {
    display: flex;
    flex-direction: row;
    gap: 1rem;
  }
</style>

{#if mergeRequest}
  <div class="merge-request-card">
    <div class="left">
      <Icon.Revision style="margin-right: 0.5rem" />
      <div>
        <div class="info-column">
          <div class="title-row">
            <p
              class="typo-text-bold typo-overflow-ellipsis"
              title={mergeRequest.id}>
              {#if mergeRequest.title}
                {mergeRequest.title}
              {:else}{mergeRequest.id}{/if}
            </p>
          </div>
          <div class="desc-row">
            <p style="color: var(--color-foreground-level-5);">
              {mergeInfo}
              by
            </p>
            {#if mergeRequest.identity}
              <Avatar
                avatarFallback={mergeRequest.identity.avatarFallback}
                size="small"
                style="display: flex; justify-content: flex-start; margin-left: 0.5rem;"
                title={mergeRequest.identity.metadata.handle}
                variant="circle" />
            {:else}
              <p style="margin-left: 0.5rem;">{mergeRequest.peerId}</p>
            {/if}
          </div>
        </div>
      </div>
    </div>

    <div class="actions-column">
      {#if showCheckout}
        <CheckoutMergeRequestButton
          id={mergeRequest.id}
          peerId={mergeRequest.peerId} />
        <AcceptMergeRequestButton id={mergeRequest.id} />
      {/if}
    </div>
  </div>
{/if}
