<script>
  import { createEventDispatcher } from "svelte";

  import { Input } from "../../Primitive";

  export let revisions = null;

  let revision;

  const dispatch = createEventDispatcher();
  const select = () => dispatch("select", revision);
</script>

<Input.Dropdown
  dataCy="revision-selector"
  style="margin-bottom: 24px"
  items={revisions}
  bind:value={revision}
  on:select={select} />
