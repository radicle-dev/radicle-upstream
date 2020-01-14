<script>
  import { getContext } from "svelte";
  import { location } from "svelte-spa-router";
  import * as path from "../path.js";
  import { Icon } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";

  const projectId = getContext("projectId");
</script>

<style>
  ul {
    height: 100%;
    align-items: center;
    display: flex;
    flex-direction: row;
  }

  li {
    margin-left: 24px;
    margin-right: 8px;
    align-items: center;
    line-height: 100%;
  }

  nav :global(li:hover p) {
    color: var(--color-purple) !important;
  }

  nav :global(li:hover .icon svg) {
    fill: var(--color-purple);
  }
  .source-tree {
    margin-top: 8px;
  }
</style>

<nav data-cy="project-sidebar">
  <ul>

    <li>
      <ProjectSidebarItem
        icon={Icon.Feed}
        title="Feed"
        href={path.projectFeed(projectId)}
        active={path.active(path.projectFeed(projectId), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Member}
        title="Members"
        href={path.projectMembers(projectId)}
        active={path.active(path.projectMembers(projectId), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Fund}
        title="Fund"
        href={path.projectFunds(projectId)}
        active={path.active(path.projectFunds(projectId), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Source}
        title="Source"
        href={path.projectSource(projectId)}
        active={path.active(path.projectSource(projectId), $location, true)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Commit}
        title="Commits"
        href={path.projectCommits(projectId)}
        active={path.active(path.projectCommits(projectId), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Branch}
        title="Branches"
        href={path.projectBranches(projectId)}
        active={path.active(path.projectBranches(projectId), $location)} />
    </li>
  </ul>
</nav>
