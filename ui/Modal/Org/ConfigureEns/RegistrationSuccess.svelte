<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsConfiguration } from "./ens-flow.types";
  import * as ensResolver from "ui/src/org/ensResolver";
  import { Emoji } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let onSubmit: () => void = () => {};
  export let ensConfiguration: EnsConfiguration;
</script>

<div>
  <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
  <Header
    title="Registration complete"
    description={`Congratulations, ` +
      `${ensConfiguration.name}.${ensResolver.DOMAIN} has successfully been ` +
      `registered with your wallet. Next, let's populate your name with ` +
      `organization metadata.`} />
  <p
    style="color: var(--color-foreground-level-5; margin: 16px 0;"
    class="typo-text-small">
    You can also do this later by selecting "Register ENS Name" and entering
    your existing name.
  </p>
  <ButtonRow
    {onSubmit}
    cancelCopy="Do this later"
    confirmCopy="Set organization metadata" />
</div>
