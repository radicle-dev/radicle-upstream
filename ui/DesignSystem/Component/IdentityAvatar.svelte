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

  $: title =
    showTitle &&
    ((titleSuffix && $identityHandleStore + titleSuffix) ||
      $identityHandleStore);
</script>

<Avatar
  {style}
  {size}
  {title}
  avatarFallback={$identityAvatarFallbackStore}
  imageUrl={$identityAvatarUrlStore}
  variant="user" />
