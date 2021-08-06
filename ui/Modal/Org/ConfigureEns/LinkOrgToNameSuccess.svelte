<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { onMount } from "svelte";
  import Emoji from "ui/DesignSystem/Emoji.svelte";

  import type { EnsConfiguration } from "./ens-flow.types";
  import type { Registration } from "ui/src/org/ensResolver";
  import * as org from "ui/src/org";

  import ButtonRow from "./shared/ButtonRow.svelte";
  import HeadlineAndDescription from "./shared/HeadlineAndDescription.svelte";

  export let onSubmit: () => void = () => {};
  export let safeAddress: string | undefined = undefined;
  export let registration: Registration | undefined = undefined;
  export let ensConfiguration: EnsConfiguration;

  onMount(() => {
    /*
    There's already a registration for the org, and that
    registration has the same name as that entered in the name
    entry step, so we can skip linking.
    */
    if (
      registration &&
      registration.name === `${ensConfiguration.name}.radicle.eth`
    ) {
      onSubmit();
    }
  });
</script>

<div>
  {#if safeAddress}
    <Emoji emoji="🤝" size="huge" style="margin-bottom: 16px" />
    <HeadlineAndDescription
      headline="Approve on Gnosis"
      description={"As a final step your organisation will have to confirm the transaction on Gnosis. After it's been approved and executed your newly registered name will start appearing across Radicle in place of your organization address!"} />
    <ButtonRow
      onSubmit={() => {
        safeAddress && org.openOnGnosisSafe(safeAddress, "transactions");
        onSubmit();
      }}
      canCancel={false}
      confirmCopy="View proposal on Gnosis" />
  {:else}
    <Emoji emoji="🎉" size="huge" style="margin-bottom: 16px" />
    <HeadlineAndDescription
      headline="That's it!"
      description={`Great, your organization now points to your new name ${ensConfiguration.name}.radicle.eth. Shortly, your name will start appearing across Radicle in place of your organization address!`} />
    <ButtonRow {onSubmit} canCancel={false} confirmCopy="Amazing, thanks!" />
  {/if}
</div>
