<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as ipc from "ui/src/ipc";
</script>

<style>
  pre {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 1rem;
    overflow: scroll;
  }
</style>

{#await ipc.getProxyLogs() then result}
  <pre>{result}</pre>
{/await}
