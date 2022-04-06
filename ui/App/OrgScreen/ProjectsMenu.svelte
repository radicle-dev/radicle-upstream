<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as org from "ui/src/org";

  import AnchorIcon from "design-system/icons/Anchor.svelte";

  import Button from "design-system/Button.svelte";
  import Tooltip from "design-system/Tooltip.svelte";

  export let orgAddress: string;
  export let gnosisSafeAddress: string;
  export let availableProjectCount: number = 0;
  export let hasPendingAnchors = false;
  export let isMultiSig: boolean;

  let tooltipMessage: string = "";
  let disabled: boolean = false;

  $: {
    disabled = availableProjectCount === 0 || hasPendingAnchors;

    if (availableProjectCount === 0) {
      tooltipMessage = "Create or track a project first";
    } else if (hasPendingAnchors) {
      tooltipMessage = "You already have pending anchors";
    } else {
      tooltipMessage = "";
    }
  }
</script>

<Tooltip value={tooltipMessage} position="left">
  <Button
    {disabled}
    variant="transparent"
    icon={AnchorIcon}
    on:click={() =>
      org.openAnchorProjectModal(orgAddress, gnosisSafeAddress, isMultiSig)}>
    Anchor a project
  </Button>
</Tooltip>
