<script lang="typescript">
  import { Button, Icon, Input } from "../../../Primitive";

  export let receivers: string[] = [];
  export let onSave: (receivers: string[]) => Promise<void>;
  export let updating = false;

  let newValue: string = "";

  function addNew(x: string) {
    console.log(`TODO(nuno): Add ${x}`);
  }
</script>

<style>
  .receivers {
    display: flex;
    align-items: center;

    margin: 1rem 0;
  }
</style>

<div class="receivers">
  <Input.Text
    disabled={updating}
    bind:value={newValue}
    placeholder="Enter an Ethereum address or a Radicle handle"
    style="min-width: 380px" />

  <Button
    disabled={updating}
    on:click={() => addNew(newValue)}
    variant="outline"
    style="margin-left: 8px; border-color: var(--color-foreground-level-3)">
    <Icon.Plus />
  </Button>
  <Button
    disabled={updating}
    on:click={() => onSave(receivers)}
    variant="secondary"
    style="margin-left: 16px;">
    Save
  </Button>
</div>
