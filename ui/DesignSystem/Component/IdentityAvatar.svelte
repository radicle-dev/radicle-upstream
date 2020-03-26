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
  export let titleSuffix = null;
</script>

<Avatar
  {style}
  {size}
  avatarFallback={$identityAvatarFallbackStore}
  imageUrl={$identityAvatarUrlStore}
  title={showTitle ? (titleSuffix ? $identityHandleStore + titleSuffix : $identityHandleStore) : null}
  variant="user" />
