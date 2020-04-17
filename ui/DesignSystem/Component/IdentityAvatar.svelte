<script>
  import { identity } from "../../src/identity.ts";
  import { Avatar } from "../Primitive";

  export let showTitle = false;
  export let style = null;
  export let size = null;
</script>

{#if $identity.status === 'SUCCESS'}
  <Avatar
    {style}
    {size}
    title={showTitle && $identity.data.metadata.handle}
    avatarFallback={$identity.data.avatarFallback}
    imageUrl={$identity.data.metadata.avatarUrl}
    variant="user" />
{/if}
