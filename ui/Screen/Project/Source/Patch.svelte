<script lang="typescript">
  import * as remote from "ui/src/remote";
  import type { Project } from "ui/src/project";
  import * as sess from "ui/src/session";
  import type { PatchDetails } from "ui/src/project/patch";
  import * as patch from "ui/src/project/patch";

  import { Remote } from "ui/DesignSystem/Component";

  import PatchLoaded from "./PatchLoaded.svelte";

  export let project: Project;
  export let id: string;
  export let peerId: string;

  const session = sess.getUnsealedFromContext();

  const patchRemote = remote.createStore<PatchDetails>();
  $: {
    remote.fetch(patchRemote, patch.getDetails(project, peerId, id));
  }
</script>

<Remote store={patchRemote} let:data={{ patch, commits }}>
  <PatchLoaded {session} {project} {patch} {commits} />
</Remote>
