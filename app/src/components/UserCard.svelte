<script>
  import { link } from "svelte-spa-router";
  import { Title } from "../DesignSystem";

  export let user = null;
  export let style = null;
</script>

<style>
  a {
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

<a href={`/users/${user.username}`} use:link {style}>
  <img src={user.avatar} alt="user-avatar" />
  <Title.Regular>{user.username}</Title.Regular>
</a>
