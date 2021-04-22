<script>
  import * as modal from "../../src/modal.ts";

  import { Icon, Button } from "../../DesignSystem/Primitive";
  import ModalAddMember from "../../Modal/Org/AddMember.svelte";
</script>

<span on:click|stopPropagation={() => modal.toggle(ModalAddMember)}>
  <Button variant="outline" icon={Icon.Plus} on:click>Add a member</Button>
</span>
