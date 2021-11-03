<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as Session from "ui/src/session";
  import { status as localPeerState } from "ui/src/localPeer";

  import Json from "./Json.svelte";
  const session = Session.unsealed();
</script>

<style>
  .container {
    gap: 2rem;
    display: flex;
    flex-direction: column;
  }
</style>

<div class="container">
  <Json title="Your identity" data={session.identity} />
  <Json title="Connection status" data={$localPeerState} />
</div>
