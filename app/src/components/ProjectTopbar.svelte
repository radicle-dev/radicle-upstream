<script>
  import { getContext } from "svelte";
  import { location } from "svelte-spa-router";
  import * as path from "../path.js";
  import { Icon } from "../DesignSystem";
  import ProjectTopbarItem from "./ProjectTopbarItem.svelte";

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

<nav data-cy="project-topbar">
  <ul>

    <li>
      <ProjectTopbarItem
        icon={Icon.Feed}
        title="Feed"
        href={path.projectFeed(projectId)}
        active={path.active(path.projectFeed(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarItem
        icon={Icon.Member}
        title="Members"
        href={path.projectMembers(projectId)}
        active={path.active(path.projectMembers(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarItem
        icon={Icon.Fund}
        title="Fund"
        href={path.projectFunds(projectId)}
        active={path.active(path.projectFunds(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarItem
        icon={Icon.Source}
        title="Source"
        href={path.projectSource(projectId)}
        active={path.active(path.projectSource(projectId), $location, true)} />
    </li>

    <li>
      <ProjectTopbarItem
        icon={Icon.Commit}
        title="Commits"
        href={path.projectCommits(projectId)}
        active={path.active(path.projectCommits(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarItem
        icon={Icon.Branch}
        title="Branches"
        href={path.projectBranches(projectId)}
        active={path.active(path.projectBranches(projectId), $location)} />
    </li>
  </ul>
</nav>
