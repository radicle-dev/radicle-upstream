<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import { User, PeerRole, PeerType } from "ui/src/project";
  import Badge from "design-system/Badge.svelte";

  export let user: User;

  let text: string | undefined = undefined;

  $: if (user.role === PeerRole.Delegate) {
    text = "delegate";
  } else if (user.type === PeerType.Local) {
    text = "you";
  } else {
    text = "";
  }
</script>

<Badge {text} />
