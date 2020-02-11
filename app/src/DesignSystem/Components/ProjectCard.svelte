<script>
  import { Icon, Text } from "../Primitives";
  import Avatar from "./Avatar.svelte";

  export let title = "";
  export let description = "";
  export let imgUrl = "";
  export let isRegistered = false;
  export let state = Icon.Check;
</script>

<style>
  .project-card {
    display: flex;
    width: 100%;
  }

  .container {
    margin: 16px 0 16px 0;
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
  }

  .first-row {
    display: flex;
    margin-bottom: 2px;
  }

  .registered {
    display: flex;
    color: var(--color-pink);
    margin-left: 16px;
  }

  .state {
    display: flex;
    align-items: center;
    margin-right: 24px;
  }
</style>

<div class="project-card">
  <Avatar imageUrl={imgUrl} variant="project" size="big" style="margin: 16px" />

  <div class="container">
    <div class="first-row">
      <Text variant="title">{title}</Text>
      {#if isRegistered}
        <div class="registered">
          <Icon.Ellipse
            style="fill: var(--color-pink); position: relative; bottom: -2px;" />
          <Text variant="smallText">Registered</Text>
        </div>
      {/if}
    </div>
    <Text>{description}</Text>
  </div>

  <div class="state">
    <svelte:component this={state} />
  </div>
</div>
