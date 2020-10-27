<script lang="typescript">
  import { push } from "svelte-spa-router";
  import { createEventDispatcher } from "svelte";

  import * as notification from "../src/notification";
  import * as path from "../src/path";
  import type { Project } from "../src/project";
  import * as remote from "../src/remote";
  import {
    projectRequest as request,
    projectSearch as store,
    reset,
    requestProject,
    searchProject,
    urnValidationStore,
  } from "../src/search";
  import { ValidationStatus } from "../src/validation";

  import { Icon, Input } from "../DesignSystem/Primitive";
  import { FollowButton, Remote } from "../DesignSystem/Component";

  let id: string;
  let value: string;

  const dispatch = createEventDispatcher();
  const urnValidation = urnValidationStore();

  const navigateToProject = (project: Project) => {
    dispatch("hide");
    push(path.projectSource(project.id));
  };
  const navigateToUntracked = () => {
    dispatch("hide");
    push(path.projectUntracked(value));
  };
  const onKeydown = (event: KeyboardEvent) => {
    switch (event.code) {
      case "Enter":
        // Navigate to project directly if present.
        if ($store.status === remote.Status.Success) {
          // FIXME(xla): Once remote/Remote offer stronger type guarantees this needs to go.
          navigateToProject(
            ($store as { status: remote.Status.Success; data: Project }).data
          );
        }
        break;
      case "Escape":
        dispatch("hide");
        break;
    }
  };
  const follow = () => {
    requestProject({ urn: value });
  };

  // Validate input entered, at the moment valid RadUrns are the only acceptable input.
  $: if (value && value.length > 0) {
    urnValidation.validate(value);
    id = value.replace("rad:git:", "");
  } else {
    urnValidation.reset();
  }
  // To support quick pasting, request the urn once valid to get tracking information.
  $: if ($urnValidation.status === ValidationStatus.Success) {
    searchProject({ urn: value });
  }
  // Reset searches if the input became invalid.
  $: if ($urnValidation.status !== ValidationStatus.Success) {
    reset();
  }
  // Fire notification when a request has been created.
  $: if ($request.status === remote.Status.Success) {
    notification.info(
      "You’ll be notified on your profile when this project has been found.",
      false,
      "View profile",
      () => {
        dispatch("hide");
        push(path.profileFollowing());
      }
    );
  }

  $: tracked = $store.status === remote.Status.Success;
  $: untracked = $store.status === remote.Status.Error;
</script>

<style>
  .container {
    width: 26.5rem;
  }

  .search-bar {
    margin-bottom: 1rem;
    position: relative;
  }

  .result {
    background: var(--color-background);
    border-radius: 0.5rem;
    height: 0;
    overflow: hidden;
    transition: height 0.3s linear;
  }

  .tracked {
    height: 5rem;
  }

  .untracked {
    height: 11rem;
  }

  .header {
    align-items: center;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .id {
    color: var(--color-foreground-level-6);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

<div class="container">
  <div class="search-bar">
    <Input.Text
      autofocus
      bind:value
      hint="v"
      inputStyle="color: var(--color-foreground-level-6);"
      on:keydown={onKeydown}
      placeholder="Have a project id? Paste it here…"
      showLeftItem
      style="border: none; border-radius: 0.5rem;"
      validation={$urnValidation}>
      <div slot="left" style="display: flex;">
        <Icon.MagnifyingGlass />
      </div>
    </Input.Text>
  </div>

  <div class="result" class:tracked class:untracked>
    <Remote {store} let:data={project}>
      <div style="padding: 1.5rem;">
        <div
          class="header typo-header-3"
          on:click={_ev => navigateToProject(project)}>
          <span class="id">{project.metadata.name}</span>
        </div>
      </div>

      <div slot="error" style="padding: 1.5rem;">
        <div class="header typo-header-3">
          <span class="id" on:click={navigateToUntracked}>{id}</span>
          <FollowButton on:follow={follow} style="margin-left: 1rem;" />
        </div>

        <p style="color: var(--color-foreground-level-6);">
          You’re not following this project yet, so there’s nothing to show
          here. Follow it and you’ll be notified as soon as it’s available.
        </p>
      </div>
    </Remote>
  </div>
</div>
