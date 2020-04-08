<script>
  import { identity } from "../../src/identity.ts";

  import { Avatar, Text } from "../Primitive";

  export let showTitle = false;
  export let style = null;
  export let size = null;
</script>

{#if $identity.status === 'NOT_ASKED'}
  <Text>NOT ASKED...</Text>
{:else if $identity.status === 'LOADING'}
  <Text>LOADING...</Text>
{:else if $identity.status === 'SUCCESS'}
  <Avatar
    {style}
    {size}
    title={showTitle && $identity.data.metadata.handle}
    avatarFallback={$identity.data.metadata.avatarFallback}
    imageUrl={$identity.data.metadata.avatarUrl}
    variant="user" />
{:else if $identity.status === 'ERROR'}
  <Text>{`Error`}</Text>
{/if}
