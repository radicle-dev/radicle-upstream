<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script context="module" lang="ts">
  import type { Project } from "ui/src/project";

  type PatchStatus =
    | { type: "loading" }
    | { type: "notReplicated" }
    | {
        type: "ok";
        patch: patch.Patch;
        commits: source.GroupedCommitsHistory;
      };

  import * as svelteStore from "svelte/store";

  import * as error from "ui/src/error";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import * as patch from "ui/src/project/patch";
  import * as source from "ui/src/source";

  const patchStatus = svelteStore.writable<PatchStatus>({ type: "loading" });
  export const patchStatusStore: svelteStore.Readable<PatchStatus> =
    patchStatus;

  const fetchExecutor = mutexExecutor.create();
  async function fetch(
    project: Project,
    peerId: string,
    id: string
  ): Promise<void> {
    patchStatus.set({ type: "loading" });
    try {
      const result = await fetchExecutor.run(async () => {
        return await patch.getDetails(project, peerId, id);
      });

      if (!result) {
        patchStatus.set({ type: "notReplicated" });
      } else {
        patchStatus.set({ type: "ok", ...result });
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
</script>

<script lang="ts">
  import type { PatchView } from "../route";

  import { onDestroy } from "svelte";

  import * as localPeer from "ui/src/localPeer";

  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import PatchLoaded from "./PatchLoaded.svelte";

  export let id: string;
  export let peerId: string;
  export let project: Project;
  export let view: PatchView;

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

{#if $patchStatus.type === "loading"}
  <Loading
    style="height: calc(100vh - var(--bigheader-height) - var(--topbar-height));" />
{:else if $patchStatus.type === "ok"}
  <PatchLoaded
    {project}
    {view}
    patch={$patchStatus.patch}
    commits={$patchStatus.commits} />
{:else if $patchStatus.type === "notReplicated"}
  <EmptyState
    emoji="👀"
    text="This patch either doesn't exist or hasn't been found yet. Once it's found, it will show up here automatically." />
{/if}
