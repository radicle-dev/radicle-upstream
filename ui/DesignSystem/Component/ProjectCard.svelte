<script>
  import { Icon, Text, Title } from "../Primitive";
  import Stat from "./Stat.svelte";

  export let title = "";
  export let description = "";
  export let isRegistered = false;
</script>

<style>
  .project-card {
    display: flex;
    width: 100%;
  }

  .container {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    width: 10%;
  }

  .first-row {
    display: flex;
    margin-bottom: 2px;
    white-space: nowrap;
  }

  .registered {
    display: flex;
    margin-left: 9px;
  }

  .stats {
    display: flex;
    align-items: center;
    margin-right: 24px;
  }
</style>

<div class="project-card">
  <div class="container">
    <div class="first-row">
      <Title>{title}</Title>
      {#if isRegistered}
        <div class="registered">
          <Icon.Badge
            style="fill: var(--color-pink); position: relative; bottom: -3px;" />
        </div>
      {/if}
    </div>
    <Text
      style="color: var(--color-gray); white-space: nowrap; overflow: hidden;
      text-overflow: ellipsis;">
      {description}
    </Text>
  </div>

  <div class="stats">
    <Stat icon={Icon.Commit} count="1.1k" style="margin-right: 24px;" />
    <Stat icon={Icon.Branch} count="60" style="margin-right: 24px;" />
    <Stat icon={Icon.Member} count="3" />
  </div>
</div>
