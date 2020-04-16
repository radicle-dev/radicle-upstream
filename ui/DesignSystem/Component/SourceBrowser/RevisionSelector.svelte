<script>
  import { Input } from "../../Primitive";

  export let revisions = null;
  export let onSelect = null;
</script>

<Input.Dropdown
  dataCy="revision-selector"
  style="margin-bottom: 24px"
  items={revisions}
  {onSelect} />
