<script>
  import { link } from "svelte-spa-router";

  import { Title } from "../../Primitive";

  import Tooltip from "./Tooltip.svelte";

  export let image = null;
  export let title = null;
  export let href = null;

  export let dataCy = null;
</script>

<style>
  img {
    width: 36px;
    height: 36px;
    border-radius: 18px;
  }

  a {
    display: flex;
  }
</style>

<a {href} use:link data-cy={dataCy}>
  <img src={image} alt="user-avatar" />
</a>

<Tooltip>
  <Title>{title}</Title>
</Tooltip>
