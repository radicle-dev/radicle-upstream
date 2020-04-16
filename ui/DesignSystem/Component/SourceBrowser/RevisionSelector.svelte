<script>
  import { Input } from "../../Primitive";

  export let revisions = null;
  export let onSelect = null;

  let revision;

  $: if (revision) onSelect(revision);
</script>

<Input.Dropdown
  dataCy="revision-selector"
  style="margin-bottom: 24px"
  items={revisions}
  bind:value={revision} />
