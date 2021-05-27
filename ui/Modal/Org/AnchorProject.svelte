<script lang="typescript">
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";

  import { Button, Emoji } from "ui/DesignSystem/Primitive";
  import { Dropdown, Modal } from "ui/DesignSystem/Component";
  import CommitTeaser from "ui/DesignSystem/Component/SourceBrowser/CommitTeaser.svelte";

  const dropdownOptions1 = [
    { value: "1", title: "Option 1" },
    {
      value: "2",
      title: "Longer option keeps going",
    },
    { value: "3", title: "Option 3" },
  ];
</script>

<style>
  .actions {
    display: flex;
    width: 100%;
    gap: 1.5rem;
    justify-content: flex-end;
  }
</style>

<Modal>
  <Emoji emoji={"🏖️"} size="huge" style="margin-bottom: 1.5rem;" />
  <h1 style="margin-bottom: 1.5rem;">Anchor a project</h1>

  <div style="width: 100%; margin-bottom: 1.5rem;">
    <Dropdown placeholder="Select a project" options={dropdownOptions1} />
  </div>
  <div style="display: flex; width: 100%; margin-bottom: 1.5rem;">
    <div style="width: 100%; margin-right: 1rem;">
      <Dropdown placeholder="Select a remote" options={dropdownOptions1} />
    </div>
    <div style="width: 100%;">
      <Dropdown placeholder="Select a branch" options={dropdownOptions1} />
    </div>
  </div>

  <CommitTeaser
    style="width: 100%; margin-bottom: 1.5rem;"
    commit={{
      author: { name: "rudolfs", email: "bla@ab.lv", avatar: "as" },
      committer: { name: "rudolfs", email: "bla@ab.lv", avatar: "as" },
      committerTime: 1234,
      summary: "as",
      sha1: "123",
      description: "ass",
    }} />
  <div class="actions">
    <Button variant="transparent" on:click={() => modal.hide()}>Cancel</Button>
    <Button
      on:click={() => {
        org.anchorProject();
      }}>Confirm in your wallet</Button>
  </div>
</Modal>
