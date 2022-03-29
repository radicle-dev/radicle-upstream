<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as format from "design-system/lib/format";

  import ArrowLeftIcon from "design-system/icons/ArrowLeft.svelte";
  import BranchBox from "./BranchBox.svelte";
  import Copyable from "ui/App/SharedComponents/Copyable.svelte";

  export let baseBranch: string;
  export let compareBranch: { id: string; peerId: string };
  export let patchUrl: string;
</script>

<style>
  .branches {
    display: flex;
    align-self: center;
    align-items: center;
    gap: 0.5rem;
  }
</style>

<div class="branches" data-cy="compare-branches">
  <BranchBox name={baseBranch} />
  <ArrowLeftIcon />
  <Copyable name="shareable link" clipboardContent={patchUrl}>
    <BranchBox
      name={`${format.shortDeviceId(compareBranch.peerId)}/${compareBranch.id}`}
      dataCy="shareable-patch-link" />
  </Copyable>
</div>
