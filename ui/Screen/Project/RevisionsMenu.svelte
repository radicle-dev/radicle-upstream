<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  variant="transparent"
  icon={Icon.Revision}
  on:click={() => console.log('event(new-revision)')}>
  New revision
</Button>
