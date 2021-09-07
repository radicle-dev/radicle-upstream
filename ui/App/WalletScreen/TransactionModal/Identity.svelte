<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { Avatar } from "ui/DesignSystem";
  import Remote from "ui/App/Remote.svelte";

  import { session } from "ui/src/session";

  export let address: string | undefined = undefined;
</script>

<style>
  .identity {
    display: flex;
    align-items: center;
    flex-direction: column;
  }

  .display {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
  }
</style>

<Remote store={session} let:data={it}>
  <div class="identity">
    <div class="display">
      <Avatar
        size="small"
        kind={{ type: "userEmoji", uniqueIdentifier: it.identity.urn }}
        style="margin-right: 10px" />
      <p class="typo-text-bold">{it.identity.metadata.handle}</p>
    </div>
    {#if address}
      <p>{address}</p>
    {/if}
  </div>
</Remote>
