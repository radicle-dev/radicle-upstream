<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { selectCommit, store } from "ui/src/screen/project/source";
  import type { CommitHeader } from "ui/src/source";

  import Remote from "ui/App/Remote.svelte";

  import History from "./SourceBrowser/History.svelte";

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
