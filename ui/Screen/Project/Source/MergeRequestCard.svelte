<script lang="typescript">
  import { CompareBranches } from "../../../DesignSystem/Component";
  import { Avatar, Icon } from "../../../DesignSystem/Primitive";

  import type { MergeRequest } from "../../../src/project/mergeRequest";

  export let mergeRequest: MergeRequest;
  export let defaultBranch: string;

  const mergeInfo = mergeRequest && mergeRequest.merged ? "Closed" : "Opened";
  const iconColor =
    mergeRequest && mergeRequest.merged
      ? "var(--color-negative)"
      : "var(--color-positive)";
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
</style>

<div class="merge-request-card">
  <div class="left">
    <Icon.Revision style={`margin-right: 0.5rem; fill: ${iconColor};`} />
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
          <p style="color: var(--color-foreground-level-5);">{mergeInfo} by</p>
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
  <CompareBranches baseBranch={defaultBranch} compareBranch={mergeRequest.id} />
</div>
