<script lang="typescript">
  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";
  import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";

  const openAnchorProjectModal = async () => {
    const projects = await org.fetchProjectDropdownOptions();
    modal.toggle(ModalAnchorProject, () => {}, { projects });
  };
</script>

<Button
  variant="transparent"
  icon={Icon.Anchor}
  on:click={openAnchorProjectModal}>
  Anchor a project
</Button>
