<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { onDestroy } from "svelte";

  import * as error from "ui/src/error";
  import * as localPeer from "ui/src/localPeer";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import type { Project } from "ui/src/project";
  import * as patch from "ui/src/project/patch";
  import * as source from "ui/src/source";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import ActionBar from "ui/App/ScreenLayout/ActionBar.svelte";
  import TabBar from "ui/App/ScreenLayout/TabBar.svelte";

  import { makeTabs } from "./tabs";
  import PatchActions from "./Patch/Actions.svelte";
  import PatchLoaded from "./Patch/Loaded.svelte";

  export let project: Project;
  export let id: string;
  export let peerId: string;
  export let view: "commits" | "discussion";
  export let patchCount: number;

  type PatchStatus =
    | { type: "loading" }
    | { type: "notReplicated" }
    | {
        type: "ok";
        patch: patch.Patch;
        commits: source.GroupedCommitsHistory;
      };

  let patchStatus: PatchStatus = { type: "loading" };

  const fetchExecutor = mutexExecutor.create();
  async function fetch(
    project: Project,
    peerId: string,
    id: string
  ): Promise<void> {
    patchStatus = { type: "loading" };
    try {
      const result = await fetchExecutor.run(async () => {
        return await patch.getDetails(project, peerId, id);
      });

      if (!result) {
        patchStatus = { type: "notReplicated" };
      } else {
        patchStatus = { type: "ok", ...result };
      }
    } catch (err: unknown) {
      notification.showException(
        new error.Error({
          message: "Failed to fetch patch",
          source: err,
        })
      );
    }
  }

  function watchPatchUpdates(): () => void {
    return localPeer.projectEvents.onValue(event => {
      if (event.urn.startsWith(project.urn)) {
        fetch(project, peerId, id);
      }
    });
  }

  const unwatchPatchUpdates = watchPatchUpdates();
  onDestroy(unwatchPatchUpdates);

  $: fetch(project, peerId, id);
</script>

<ActionBar>
  <TabBar
    tabs={makeTabs({
      projectUrn: project.urn,
      activeViewType: "patches",
      patchCount,
      commitCount: project.stats.commits,
    })} />
  <div style="margin-left: auto" />
  {#if patchStatus.type === "ok"}
    <PatchActions {project} patch={patchStatus.patch} />
  {/if}
</ActionBar>

{#if patchStatus.type === "loading"}
  <Loading
    style="height: calc(100vh - var(--bigheader-height) - var(--topbar-height));" />
{:else if patchStatus.type === "ok"}
  <PatchLoaded
    {project}
    {view}
    patch={patchStatus.patch}
    commits={patchStatus.commits} />
{:else if patchStatus.type === "notReplicated"}
  <EmptyState
    emoji="👀"
    text="This patch either doesn't exist or hasn't been found yet. Once it's found, it will show up here automatically." />
{/if}
