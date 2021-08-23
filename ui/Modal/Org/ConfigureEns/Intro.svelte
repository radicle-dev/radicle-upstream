<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type * as ethers from "ethers";

  import * as ensRegistrar from "ui/src/org/ensRegistrar";
  import * as ipc from "ui/src/ipc";
  import * as modal from "ui/src/modal";

  import { Button, Modal } from "ui/DesignSystem";

  export let fee: ethers.BigNumber;
  export let onSubmit: () => void;
</script>

<Modal emoji="📇" title={`Register your ENS name`}>
  <svelte:fragment slot="description">
    Your ENS name allows linking your org with a name, logo, URL and social
    media profiles. The registration costs
    {ensRegistrar.formatFee(fee)} RAD, and you'll also need sufficient ETH to cover
    transaction costs.
    <!-- svelte-ignore a11y-missing-attribute -->
    <a
      class="typo-link"
      on:click={() => {
        ipc.openUrl("http://radicle.xyz/buy-rad.html");
      }}>
      Buy RAD
    </a>
  </svelte:fragment>

  <svelte:fragment slot="buttons">
    <Button
      variant="transparent"
      on:click={() => {
        modal.hide();
      }}>Cancel</Button>
    <Button on:click={onSubmit}>Let’s go</Button>
  </svelte:fragment>
</Modal>
