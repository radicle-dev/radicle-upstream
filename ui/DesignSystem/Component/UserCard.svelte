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

  img {
    width: 24px;
    height: 24px;
    border-radius: 12px;
    margin-right: 8px;
  }
</style>

<span class="user" {style}>
  <img src={user.avatar} alt="user-avatar" />
  <Title>{user.username}</Title>
</span>
