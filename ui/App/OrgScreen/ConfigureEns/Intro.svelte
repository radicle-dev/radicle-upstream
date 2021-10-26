<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as ethers from "ethers";

  import * as ethereum from "ui/src/ethereum";
  import * as modal from "ui/src/modal";

  import Button from "design-system/Button.svelte";
  import Modal from "ui/App/ModalLayout/Modal.svelte";

  export let fee: ethers.BigNumber;
  export let onSubmit: () => void;
</script>

<Modal emoji="📇" title={`Register your ENS name`}>
  <svelte:fragment slot="description">
    Your ENS name allows linking your org with a name, logo, URL and social
    media profiles. The registration costs {ethereum.formatTokenAmount(fee)}
    RAD, and you'll also need sufficient ETH to cover transaction costs.
    <a
      class="typo-link"
      href="https://docs.radicle.xyz/docs/connecting-to-ethereum/obtaining-rad"
      >Buy RAD</a>
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
