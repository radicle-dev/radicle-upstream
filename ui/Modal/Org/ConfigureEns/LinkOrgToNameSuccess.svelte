<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { EnsConfiguration } from "./ens-flow.types";

  import * as ensResolver from "ui/src/org/ensResolver";
  import * as org from "ui/src/org";

  import { Emoji } from "ui/DesignSystem";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import Header from "./shared/Header.svelte";

  export let onSubmit: () => void = () => {};
  export let safeAddress: string | undefined = undefined;
  export let ensConfiguration: EnsConfiguration;
</script>

<div>
  {#if safeAddress}
    <Emoji emoji="🤝" size="huge" style="margin-bottom: 16px" />
    <Header
      title="Approve on Gnosis"
      description={"As a final step your organisation will have to confirm " +
        "the transaction on Gnosis. After it's been approved and executed " +
        "your newly registered name will start appearing across Radicle in " +
        "place of your organization address!"} />
    <ButtonRow
      onSubmit={() => {
        safeAddress && org.openOnGnosisSafe(safeAddress, "transactions");
        onSubmit();
      }}
      canCancel={false}
      confirmCopy="View proposal on Gnosis" />
  {:else}
    <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
    <Header
      title="That's it!"
      description={`Great, your organization now points to your new name ` +
        `${ensConfiguration.name}.${ensResolver.DOMAIN}. Shortly, your name ` +
        `will start appearing across Radicle in place of your organization ` +
        `address!`} />
    <ButtonRow {onSubmit} canCancel={false} confirmCopy="Amazing, thanks!" />
  {/if}
</div>
