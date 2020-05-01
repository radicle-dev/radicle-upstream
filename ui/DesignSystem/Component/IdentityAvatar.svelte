<script>
  import { Avatar } from "../Primitive";

  export let identity = null;
  export let showTitle = false;
  export let style = null;
  export let size = null;
</script>

<Avatar
  {style}
  {size}
  title={showTitle && identity.metadata.handle}
  avatarFallback={identity.avatarFallback}
  imageUrl={identity.metadata.avatarUrl}
  variant="circle" />
