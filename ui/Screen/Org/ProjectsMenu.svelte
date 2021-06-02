<script lang="typescript">
  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import * as modal from "ui/src/modal";
  import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";

  const openAnchorProjectModal = () => {
    modal.toggle(ModalAnchorProject, () => {}, {});
  };
</script>

<Button
  variant="transparent"
  icon={Icon.Anchor}
  on:click={openAnchorProjectModal}>
  Anchor a project
</Button>
