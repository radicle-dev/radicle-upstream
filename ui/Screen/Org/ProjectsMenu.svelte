<script lang="typescript">
  import * as org from "ui/src/org";
  import { Button, Icon } from "ui/DesignSystem/Primitive";

  export let orgAddress: string;
  export let gnosisSafeAddress: string;
</script>

<Button
  variant="transparent"
  icon={Icon.Anchor}
  on:click={() => org.openAnchorProjectModal(orgAddress, gnosisSafeAddress)}>
  Anchor a project
</Button>
