<script lang="typescript">
  import { getContext } from "svelte";
  import * as remote from "../../../src/remote";
  import MergeRequestLoaded from "./MergeRequestLoaded.svelte";

  import type { Project } from "../../../src/project";
  import type { UnsealedSession } from "../../../src/session";
  import type { MergeRequestDetails } from "../../../src/project/mergeRequest";
  import * as mergeRequest from "../../../src/project/mergeRequest";

  import { Remote } from "../../../DesignSystem/Component";

  export let project: Project;
  export let params: {
    urn: string;
    peerId: string;
    id: string;
  };

  const session = getContext("session") as UnsealedSession;

  const mergeRequestRemote = remote.createStore<MergeRequestDetails>();
  $: {
    remote.fetch(
      mergeRequestRemote,
      mergeRequest.getDetails(
        session.identity.peerId,
        project,
        params.peerId,
        params.id
      )
    );
  }
</script>

<Remote store={mergeRequestRemote} let:data={{ mergeRequest, commits }}>
  <MergeRequestLoaded {session} {project} {mergeRequest} {commits} />
</Remote>
