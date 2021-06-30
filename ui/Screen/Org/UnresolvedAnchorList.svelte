<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { fade } from "svelte/transition";

  import type * as project from "ui/src/project";

  import * as notification from "ui/src/notification";
  import * as search from "ui/src/search";
  import * as router from "ui/src/router";
  import {
    ProjectAnchorPopover,
    FollowToggle,
    Hoverable,
    List,
    RadicleId,
  } from "ui/DesignSystem";

  export let anchors: project.Anchor[];

  const onFollow = (projectId: string) => {
    search.requestProject(projectId);
    router.push({ type: "profile", activeTab: "following" });
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
        <RadicleId urn={anchor.projectId} showIcon={false} />
        <ProjectAnchorPopover {anchor} />
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
