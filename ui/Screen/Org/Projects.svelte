<script lang="typescript">
  import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";
  import * as modal from "ui/src/modal";
  import * as org from "ui/src/org";

  import { EmptyState } from "ui/DesignSystem/Component";
  import { Variant as IllustrationVariant } from "ui/src/illustration";

  const orgProjectTabStore = org.orgProjectTabStore;
</script>

{#if $orgProjectTabStore !== null}
  {$orgProjectTabStore.map(p => {
    return p.anchor.id;
  })}
{:else}
  <EmptyState
    illustration={IllustrationVariant.Plant}
    text="Get started by anchoring your organization’s first project with the radicle gnosis safe app."
    primaryActionText="Anchor with Gnosis Safe"
    on:primaryAction={() => {
      modal.toggle(ModalAnchorProject);
    }} />
{/if}
