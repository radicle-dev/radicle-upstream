<script>
  import { push } from "svelte-spa-router";
  import { Icon, Text, Title } from "../Primitive";
  import AdditionalActionsDropdown from "./AdditionalActionsDropdown.svelte";
  import Stat from "./Stat.svelte";
  import { registerProject } from "../../lib/path.js";

  export let projectId = null;
  export let title = "";
  export let description = "";
  export let isRegistered = false;

  const dropdownMenuItems = [
    {
      title: "Share project",
      icon: Icon.ArrowUp,
      event: () => console.log(`event(share-project(${projectId}))`)
    },
    {
      title: "Register project",
      icon: Icon.Register,
      event: () => push(registerProject(projectId))
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

  <div class="right">
    <Stat icon={Icon.Commit} count="1.1k" style="margin-right: 32px;" />
    <Stat icon={Icon.Branch} count="60" style="margin-right: 32px;" />
    <Stat icon={Icon.Member} count="3" style="margin-right: 44px;" />

    <AdditionalActionsDropdown menuItems={dropdownMenuItems} />
  </div>
</div>
