<script lang="typescript">
  import { fade } from "svelte/transition";
  import { push } from "svelte-spa-router";

  import ModalSearch from "../../Modal/Search.svelte";

  import { FADE_DURATION } from "../../src/config";
  import * as modal from "../../src/modal";
  import * as path from "../../src/path";
  import { following as store, fetchFollowing } from "../../src/profile";
  import * as proxy from "../../src/proxy";
  import type { Project } from "../../src/project";
  import * as sess from "../../src/session";
  import type { Urn } from "../../src/urn";

  import {
    EmptyState,
    Hoverable,
    List,
    ProjectList,
    Remote,
    RadicleId,
    FollowToggle,
  } from "../../DesignSystem/Component";
  import { Button, Icon } from "../../DesignSystem/Primitive";

  const session = sess.getUnsealedFromContext();
  const onCancel = (urn: Urn): void => {
    proxy.client.project.requestCancel(urn).then(fetchFollowing);
  };
  const onSelect = ({ detail: project }: { detail: Project }) => {
    push(path.project(project.urn));
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
    min-height: 6rem;

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
            These projects haven't been found in your network yet or don't
            exist.
          </p>
        </div>
        <List items={data.requests} let:item={request} key="urn">
          <Hoverable let:hovering={hover} style="flex: 1;">
            <div
              data-cy="undiscovered-project"
              class="undiscovered-project"
              out:fade|local={{ duration: FADE_DURATION }}>
              <RadicleId urn={request.urn} />
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
          modal.toggle(ModalSearch);
        }}
        icon={Icon.MagnifyingGlass}
        variant="outline">
        Look for a project
      </Button>
    </div>

    <div slot="empty">
      <EmptyState
        text="You're not following any projects yet."
        emoji="🐎"
        primaryActionText="Look for a project"
        on:primaryAction={() => {
          modal.toggle(ModalSearch);
        }} />
    </div>
  </Remote>
</div>
