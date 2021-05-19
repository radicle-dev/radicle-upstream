<script lang="typescript">
  import ModalAnchorProject from "ui/Modal/Org/AnchorProject.svelte";
  import * as modal from "ui/src/modal";

  import { EmptyState } from "ui/DesignSystem/Component";
  import { Variant as IllustrationVariant } from "ui/src/illustration";

</script>

<EmptyState
  illustration={IllustrationVariant.Plant}
  text="Get started by anchoring your organization’s first project with the radicle gnosis safe app."
  primaryActionText="Anchor with Gnosis Safe"
  on:primaryAction={() => {
    modal.toggle(ModalAnchorProject);
  }} />
