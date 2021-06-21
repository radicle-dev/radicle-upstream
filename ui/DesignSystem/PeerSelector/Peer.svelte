<script lang="typescript">
  import { BadgeType } from "ui/src/badge";
  import { PeerRole, PeerType } from "ui/src/project";
  import type { User } from "ui/src/project";

  import Avatar from "ui/DesignSystem/Avatar.svelte";
  import Badge from "ui/DesignSystem/Badge.svelte";

  export let peer: User;
</script>

<style>
  .peer {
    display: flex;
    justify-content: flex-start;
  }

  p.name,
  p.badge {
    margin-left: 0.5rem;
  }
</style>

<div class="peer" data-peer-handle={peer.identity.metadata.handle}>
  <Avatar
    avatarFallback={peer.identity.avatarFallback}
    size="small"
    variant="circle" />
  <p class="name typo-text-bold typo-overflow-ellipsis">
    {peer.identity.metadata.handle}
  </p>
  {#if peer.role === PeerRole.Maintainer}
    <p class="badge">
      <Badge variant={BadgeType.Maintainer} />
    </p>
  {:else if peer.type === PeerType.Local}
    <p class="badge">
      <Badge variant={BadgeType.You} />
    </p>
  {/if}
</div>
