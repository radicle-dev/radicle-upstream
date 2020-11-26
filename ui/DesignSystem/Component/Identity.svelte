<script lang="typescript">
  import { Avatar } from "../Primitive";
  import { Remote } from "./";

  import { session } from "../../src/session";
</script>

<style>
  .identity {
    display: flex;
    align-items: center;
  }
</style>

<Remote store={session} let:data={it}>
  <p class="identity typo-text-bold">
    <Avatar
      size="small"
      avatarFallback={it.identity.avatarFallback}
      variant="circle"
      style="margin-right: 10px" />
    {it.identity.metadata.handle}
  </p>
</Remote>
