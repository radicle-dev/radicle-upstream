<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Blob } from "ui/src/screen/project/source";

  import FileSource from "../FileSource.svelte";

  export let view: Blob;
</script>

<FileSource
  blob={view.blob}
  commit={view.blob.info.lastCommit}
  on:root
  on:select />
