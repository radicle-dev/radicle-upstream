<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { Project } from "ui/src/project";

  import { onDestroy } from "svelte";

  import * as error from "ui/src/error";
  import * as localPeer from "ui/src/localPeer";
  import * as mutexExecutor from "ui/src/mutexExecutor";
  import * as notification from "ui/src/notification";
  import * as patch from "ui/src/project/patch";
  import * as source from "ui/src/source";

  import PatchLoaded from "./PatchLoaded.svelte";
  import EmptyState from "ui/App/SharedComponents/EmptyState.svelte";

  export let id: string;
  export let peerId: string;
  export let project: Project;

  let patchStatus:
    | { type: "loading" }
    | {
        type: "ok";
        patch: patch.Patch;
        commits: source.GroupedCommitsHistory;
      } = { type: "loading" };

  const fetchExecutor = mutexExecutor.create();
  async function fetch(
    project: Project,
    peerId: string,
    id: string
  ): Promise<void> {
    try {
      const result = await fetchExecutor.run(async () => {
        return await patch.getDetails(project, peerId, id);
      });

      if (!result) {
        patchStatus = { type: "loading" };
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

{#if patchStatus.type === "loading"}
  <EmptyState
    emoji="👀"
    text="This patch either doesn't exist or hasn't been found yet. Once it's found, it will show up here automatically." />
{:else if patchStatus.type === "ok"}
  <PatchLoaded
    {project}
    patch={patchStatus.patch}
    commits={patchStatus.commits} />
{/if}
