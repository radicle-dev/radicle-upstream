<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Emoji from "ui/DesignSystem/Emoji.svelte";

  import type { EnsConfiguration } from "../ens-flow.types";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

  export let onSubmit: () => void = () => {};

  export let ensConfiguration: EnsConfiguration;
</script>

<div>
  <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
  <HeadlineAndDescription
    headline="That's it!"
    description={`Great, your organization now points to your new name ${ensConfiguration.name}.radicle.eth. Shortly, your name will start appearing across Radicle in place of your organization address!`} />
  <ButtonRow {onSubmit} canCancel={false} confirmCopy="Amazing, thanks!" />
</div>
