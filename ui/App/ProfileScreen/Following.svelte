<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Urn } from "ui/src/urn";
  import type { Project } from "ui/src/project";

  import { fade } from "svelte/transition";

  import { following as store, fetchFollowing } from "ui/src/profile";
  import * as Session from "ui/src/session";
  import * as modal from "ui/src/modal";
  import * as proxy from "ui/src/proxy";
  import * as router from "ui/src/router";

  import {
    Button,
    CopyableIdentifier,
    FollowToggle,
    Hoverable,
    Icon,
    List,
  } from "ui/DesignSystem";

  import EmptyState from "ui/App/ScreenLayout/EmptyState.svelte";
  import Remote from "ui/App/Remote.svelte";
  import SearchModal from "ui/App/SearchModal.svelte";
  import ProjectList from "./ProjectList.svelte";

  const FADE_DURATION = 200;
  const session = Session.unsealed();
  const onCancel = (urn: Urn): void => {
    proxy.client.project.requestCancel(urn).then(fetchFollowing);
  };
  const onSelect = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
    });
  };

  fetchFollowing();
</script>

<style>
  .container {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .header {
    display: flex;
    margin: 1.5rem 3rem 0.5rem;
  }

  .undiscovered-project {
    padding: 1.5rem;
    flex: 1;
    height: 4.375rem;

    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .undiscovered-project:hover {
    background-color: var(--color-background);
  }

  .search-box {
    display: flex;
    gap: 1rem;
    justify-content: center;
    align-items: center;
    margin-top: 1.5rem;
  }
</style>

<div class="container" data-cy="following-tab-contents">
  <Remote {store} let:data>
    <ProjectList
      projects={data.follows}
      userUrn={session.identity.urn}
      on:select={onSelect} />

    {#if data.requests.length > 0}
      <div out:fade|local={{ duration: FADE_DURATION }}>
        <div class="header">
          <p class="typo-text-bold">Still looking…&nbsp;</p>
          <p style="color: var(--color-foreground-level-6);">
            These projects haven’t been found yet. Make sure you’re connected to
            the seed node they’re on.
          </p>
        </div>
        <List items={data.requests} let:item={request} key="urn">
          <Hoverable let:hovering={hover} style="flex: 1;">
            <div
              data-cy="undiscovered-project"
              class="undiscovered-project"
              out:fade|local={{ duration: FADE_DURATION }}>
              <CopyableIdentifier value={request.urn} kind="radicleId" />
              {#if hover}
                <div transition:fade={{ duration: FADE_DURATION }}>
                  <FollowToggle
                    following
                    on:unfollow={() => onCancel(request.urn)} />
                </div>
              {/if}
            </div>
          </Hoverable>
        </List>
      </div>
    {/if}

    <div class="search-box">
      <p style="color: var(--color-foreground-level-5);">
        Follow a new project
      </p>
      <Button
        on:click={() => {
          modal.toggle(SearchModal);
        }}
        icon={Icon.MagnifyingGlass}
        variant="outline">
        Look for a project
      </Button>
    </div>

    <div slot="empty">
      <EmptyState
        text="You’re not following any projects yet."
        emoji="🐎"
        primaryActionText="Look for a project"
        on:primaryAction={() => {
          modal.toggle(SearchModal);
        }} />
    </div>
  </Remote>
</div>
