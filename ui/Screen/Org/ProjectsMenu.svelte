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
  export let disabled = false;
  export let pendingAnchor = false;

  let tooltipMessage: string = "";

  $: if (disabled && !pendingAnchor) {
    tooltipMessage = "Create or follow a project first";
  } else if (disabled && pendingAnchor) {
    tooltipMessage = "You already have pending anchors";
  } else if (!disabled && !pendingAnchor) {
    tooltipMessage = "";
  }
</script>

<Tooltip value={tooltipMessage} position={style.CSSPosition.Left}>
  <Button
    {disabled}
    variant="transparent"
    icon={Icon.Anchor}
    on:click={() => org.openAnchorProjectModal(orgAddress, gnosisSafeAddress)}>
    Anchor a project
  </Button>
</Tooltip>
