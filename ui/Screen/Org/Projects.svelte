<script>
  import { EmptyState } from "../../DesignSystem/Component";
  import { Variant as IllustrationVariant } from "../../src/illustration.ts";
</script>

<EmptyState
  illustration={IllustrationVariant.Plant}
  text="Get started by anchoring your organization’s first project with the radicle gnosis safe app."
  primaryActionText="Anchor with Gnosis Safe"
  on:primaryAction={() => {
    window.location.href = 'https://gnosis-safe.io/';
  }} />
