<script lang="typescript">
  import { Avatar } from "../Primitive";
  import Remote from "./Remote.svelte";

  import { session } from "../../src/session";

  export let address = "";
</script>

<style>
  .identity {
    display: flex;
    align-items: center;

    flex-direction: column;
  }

  .display {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
  }
</style>

<Remote store={session} let:data={it}>
  <div class="identity">
    <div class="display">
      <Avatar
        size="small"
        avatarFallback={it.identity.avatarFallback}
        variant="circle"
        style="margin-right: 10px" />
      <p class="typo-text-bold">{it.identity.metadata.handle}</p>
    </div>
    <p>{address}</p>
  </div>
</Remote>
