<script>
  import { Title } from "../Primitive";

  export let user = null;
  export let style = null;
</script>

<style>
  .user {
    display: flex;
    align-items: center;
  }
</style>

<span class="user" {style}>
  <Title>{user.username}</Title>
</span>
