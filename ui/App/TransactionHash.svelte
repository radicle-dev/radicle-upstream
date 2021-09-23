<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as org from "ui/src/org";
  import { format } from "ui/DesignSystem";

  export let hash: string;
</script>

<a class="typo-link" href={org.etherscanUrl(hash)}>{format.shortEthTx(hash)}</a>
