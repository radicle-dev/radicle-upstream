<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as org from "ui/src/org";
  import * as format from "ui/src/format";

  export let hash: string;

  import ExternalLink from "ui/App/ExternalLink.svelte";
</script>

<ExternalLink url={org.etherscanUrl(hash)}>
  {format.shortEthTx(hash)}
</ExternalLink>
