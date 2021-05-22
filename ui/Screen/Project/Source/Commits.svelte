<script lang="typescript">
  import { selectCommit, store } from "../../../src/screen/project/source";
  import type { CommitHeader } from "../../../src/source";

  import { Remote } from "../../../DesignSystem/Component";

  import History from "../../../DesignSystem/Component/SourceBrowser/History.svelte";

  const onSelect = ({ detail: commit }: { detail: CommitHeader }) => {
    selectCommit(commit);
  };
</script>

<style>
  .commits-page {
    margin: 0 auto 6rem;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 2rem var(--content-padding) 0;
  }
</style>

<div class="commits-page" data-cy="commits-page">
  <Remote {store} let:data={{ history }}>
    <History {history} on:select={onSelect} />
  </Remote>
</div>
