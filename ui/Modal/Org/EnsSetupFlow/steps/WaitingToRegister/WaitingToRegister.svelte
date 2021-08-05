<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsConfiguration } from "../../ens-flow.types";
  import BlockTimer from "./components/BlockTimer.svelte";

  export let onSubmit: () => void = () => {};
  export let ensConfiguration: Partial<EnsConfiguration>;
</script>

<div style="color: var(--color-foreground-level-5);">
  <BlockTimer
    onFinish={onSubmit}
    requiredBlocks={ensConfiguration.minAge}
    startBlock={ensConfiguration.commitmentBlock} />
  <h3 style="margin-top: 24px">Awaiting registration commitment...</h3>
  <p style="margin: 24px 0">
    This will take about one minute. The waiting period is required to ensure
    another person hasn’t tried to register the same name.
  </p>
</div>
