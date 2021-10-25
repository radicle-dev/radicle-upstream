<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { fade } from "svelte/transition";

  import type * as project from "ui/src/project";

  import * as notification from "ui/src/notification";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";

  import {
    CopyableIdentifier,
    FollowToggle,
    Hoverable,
    List,
  } from "ui/DesignSystem";

  import ProjectAnchorHovercard from "ui/App/SharedComponents/ProjectAnchorHovercard.svelte";

  export let anchors: project.Anchor[];

  const onFollow = (projectId: string) => {
    proxy.client.project.requestSubmit(projectId);
    router.push({ type: "profile" });
    notification.info({
      message: `Added ${projectId} to the queue`,
      showIcon: true,
    });
  };
</script>

<style>
  .list-item {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding: 1.375rem 1.5rem;
    align-items: center;
    min-width: 0;
    height: 4.375rem;
  }
  .anchor-row {
    display: flex;
    white-space: nowrap;
    width: -webkit-fill-available;
    color: var(--color-foreground-level-6);
  }
</style>

<List
  dataCy="project-list"
  items={anchors}
  let:item={anchor}
  styleHoverState={false}
  style="margin: 0 auto;">
  <Hoverable let:hovering={hover} style="flex: 1">
    <div class="list-item" data-cy={`project-list-entry-${anchor.id}`}>
      <div class="typo-text anchor-row">
        <CopyableIdentifier value={anchor.projectId} kind="radicleId" />
        <ProjectAnchorHovercard {anchor} />
      </div>
      {#if hover}
        <div transition:fade|local={{ duration: 200 }}>
          <FollowToggle
            on:follow={() => {
              onFollow(anchor.projectId);
            }} />
        </div>
      {/if}
    </div>
  </Hoverable>
</List>
