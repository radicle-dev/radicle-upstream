<script>
  import * as modal from "../../src/modal.ts";
  import * as path from "../../src/path.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<span on:click|stopPropagation={() => modal.toggle(path.newProject())}>
  <Button
    dataCy="new-project-button"
    variant="outline"
    icon={Icon.Plus}
    on:click>
    New project
  </Button>
</span>
