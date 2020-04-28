<script>
  import { push } from "svelte-spa-router";
  import { Icon, Text, Title } from "../Primitive";
  import AdditionalActionsDropdown from "./AdditionalActionsDropdown.svelte";
  import Stat from "./Stat.svelte";
  import { registerExistingProject } from "../../lib/path.js";

  export let projectId = null;
  export let registrarId = null;
  export let title = null;
  export let description = null;
  export let isRegistered = false;
  export let stats = { branches: -1, commits: -1, contributors: -1 };

  const dropdownMenuItems = [
    {
      title: "Register project",
      icon: Icon.Register,
      event: () => push(registerExistingProject(projectId, registrarId))
    }
  ];
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

  .right {
    display: flex;
    align-items: center;
  }
</style>

<div class="project-card">
  <div class="container">
    <div class="first-row">
      <Title>{title}</Title>
      {#if isRegistered}
        <div class="registered">
          <Icon.Badge
            style="fill: var(--color-primary); position: relative; bottom: -3px;" />
        </div>
      {/if}
    </div>
    <Text
      style="color: var(--color-foreground-level-5); white-space: nowrap;
      overflow: hidden; text-overflow: ellipsis;">
      {description}
    </Text>
  </div>

  <div class="right">
    <Stat
      icon={Icon.Commit}
      count={stats.commits}
      style="margin-right: 32px;" />
    <Stat
      icon={Icon.Branch}
      count={stats.branches}
      style="margin-right: 32px;" />
    <Stat
      icon={Icon.Member}
      count={stats.contributors}
      style="margin-right: 44px;" />

    <AdditionalActionsDropdown
      headerTitle={projectId}
      menuItems={dropdownMenuItems} />
  </div>
</div>
