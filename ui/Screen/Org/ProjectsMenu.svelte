<script>
  import * as modal from "../../src/modal.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
  import ModalAnchorProject from "../../Modal/Org/AnchorProject.svelte";
</script>

<span on:click|stopPropagation={() => modal.toggle(ModalAnchorProject)}>
  <Button variant="outline" icon={Icon.Anchor} on:click>
    Anchor a project
  </Button>
</span>
