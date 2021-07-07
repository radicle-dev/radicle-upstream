<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as style from "ui/src/style";
  import * as org from "ui/src/org";
  import { Button, Icon, Tooltip } from "ui/DesignSystem";

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
      tooltipMessage = "Create or follow a project first";
    } else if (hasPendingAnchors) {
      tooltipMessage = "You already have pending anchors";
    } else {
      tooltipMessage = "";
    }
  }
</script>

<Tooltip value={tooltipMessage} position={style.CSSPosition.Left}>
  <Button
    {disabled}
    variant="transparent"
    icon={Icon.Anchor}
    on:click={() =>
      org.openAnchorProjectModal(orgAddress, gnosisSafeAddress, isMultiSig)}>
    Anchor a project
  </Button>
</Tooltip>
