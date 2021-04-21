<script>
  import * as modal from "../../src/modal.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
  import ModalNewProject from "../../Modal/NewProject.svelte";
</script>

<span on:click|stopPropagation={() => modal.toggle(ModalNewProject)}>
  <Button
    dataCy="new-project-button"
    variant="outline"
    icon={Icon.Plus}
    on:click>
    New project
  </Button>
</span>
