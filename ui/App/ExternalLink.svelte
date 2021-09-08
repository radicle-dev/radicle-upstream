<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as ipc from "ui/src/ipc";

  export let url: string;
</script>

<style>
  .title {
    text-decoration: underline;
    margin-right: 0.1rem;
  }

  .arrow {
    vertical-align: text-top;
  }
</style>

<span
  class="typo-link"
  style="text-decoration: none;"
  on:click={() => {
    ipc.openUrl(url);
  }}>
  <span class="title"
    >{#if $$slots.default}<slot />{:else}{url}{/if}</span
  ><span class="arrow">↗</span></span>
