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
          <Icon.CheckCircle
            style="fill: var(--color-pink); position: relative; bottom: -2px;
            width: 16px; height: 16px;" />
        </div>
      {/if}
    </div>
    <Text style="color: var(--color-gray);">{description}</Text>
  </div>

  <div class="stats">
    <Stat icon={Icon.Commit} count="1.1k" style="margin-right: 24px;" />
    <Stat icon={Icon.Branch} count="60" style="margin-right: 24px;" />
    <Stat icon={Icon.Member} count="3" />
  </div>
</div>
