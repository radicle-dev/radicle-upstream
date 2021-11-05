<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as proxy from "ui/src/proxy";
  import Json from "./Json.svelte";
</script>

{#await proxy.client.diagnosticsGet() then result}
  <Json title="Storage locations" data={result} />
{/await}
