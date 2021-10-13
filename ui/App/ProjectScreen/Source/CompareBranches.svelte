<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { Icon } from "ui/DesignSystem";
  import BranchBox from "./BranchBox.svelte";

  export let baseBranch: string;
  export let compareBranch: string;
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
  <Icon.ArrowLeft />
  <BranchBox name={compareBranch} />
</div>
