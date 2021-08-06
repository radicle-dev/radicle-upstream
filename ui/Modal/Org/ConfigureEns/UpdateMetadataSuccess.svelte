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
    title="Metadata successfully updated"
    description={`Great, your name ` +
      `${ensConfiguration.name}.${ensResolver.DOMAIN} has successfully been ` +
      `updated with your supplied metadata.`} />
  <ButtonRow {onSubmit} canCancel={false} confirmCopy="Continue" />
</div>
