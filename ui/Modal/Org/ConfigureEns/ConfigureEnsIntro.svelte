<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as ethers from "ethers";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ensResolver from "ui/src/org/ensResolver";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let fee: ethers.BigNumber;
  export let onSubmit: () => void;
</script>

<div>
  <Header
    title={`Register your ${ensResolver.DOMAIN} name`}
    description={`Your ${ensResolver.DOMAIN} name allows linking your ` +
      `organization with a name, logo, URL and social media profiles.`}
    style="margin-bottom: 1rem;" />

  ENS name registration costs {ensRegistrar.formatFee(fee)} RAD, you'll also need
  sufficient ETH to cover transaction costs.
  <ButtonRow {onSubmit} confirmCopy="Let's go" />
</div>
