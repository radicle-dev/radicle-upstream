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

  import { Modal } from "ui/DesignSystem";
  import ButtonRow from "./ButtonRow.svelte";

  export let fee: ethers.BigNumber;
  export let onSubmit: () => void;
</script>

<Modal
  emoji="📇"
  title={`Register your ${ensResolver.DOMAIN} name`}
  desc={`Your ${
    ensResolver.DOMAIN
  } name allows linking your organization with a name, logo, URL and social media profiles. The registration costs ${ensRegistrar.formatFee(
    fee
  )} RAD, you'll also need sufficient ETH funds to cover transaction costs.`}>
  <ButtonRow {onSubmit} confirmCopy="Let's go" />
</Modal>
