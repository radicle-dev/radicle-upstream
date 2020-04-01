<script>
  import { Icon, Button } from "../../DesignSystem/Primitive";
</script>

<Button
  size="small"
  variant="transparent"
  icon={Icon.Revisions}
  on:click={() => console.log('event(new-revision)')}>
  New revision
</Button>
