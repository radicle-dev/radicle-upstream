<script lang="typescript">
  import * as style from "ui/src/style";
  import * as org from "ui/src/org";
  import { Button, Icon } from "ui/DesignSystem";
  import { Tooltip } from "ui/DesignSystem";

  export let orgAddress: string;
  export let gnosisSafeAddress: string;
  export let disabled: boolean = false;

  $: tooltipMessage = disabled ? "Create or follow a project first" : "";
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
