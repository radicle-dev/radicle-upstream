<script>
  import { getContext } from "svelte";
  import { location } from "svelte-spa-router";
  import * as path from "../path.js";
  import { Icon } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
  import Folder from "./Folder.svelte";

  const projectId = getContext("projectId");
</script>

<style>
  nav {
    /* width: var(--project-sidebar-width); */
    /* height: 100%; */
    /* width: calc(100vw - var(--slim-sidebar-width)); */
    display: flex;
    /* position: fixed; */
    flex-direction: column;
    /* justify-content: space-between; */
    /* padding: 44px 20px 0 24px; */
    /* overflow-y: auto; */
  }

  ul {
    display: flex;
    flex-direction: row;
  }

  li {
    /* margin-bottom: 14px; */
    /* display: inline; */
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
        icon={Icon.Home}
        title="Overview"
        href={path.projectOverview(projectId)}
        active={path.active(path.projectOverview(projectId), $location)} />
    </li>

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
        active={path.active(path.projectSource(projectId), $location, true)}>

        <!-- <div class="source-tree" data-cy="source-tree">
          <Folder />
        </div> -->
      </ProjectSidebarItem>
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
