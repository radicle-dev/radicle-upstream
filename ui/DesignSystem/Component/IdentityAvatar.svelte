<script>
  import { Avatar } from "../Primitive";
  import {
    identityAvatarUrlStore,
    identityAvatarFallbackStore,
    identityHandleStore
  } from "../../store/identity.js";

  export let showTitle = false;
  export let style = null;
  export let size = null;
</script>

<Avatar
  {style}
  {size}
  title={showTitle && $identityHandleStore}
  avatarFallback={$identityAvatarFallbackStore}
  imageUrl={$identityAvatarUrlStore}
  variant="user" />
