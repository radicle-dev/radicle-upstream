<script>
  import { location } from "svelte-spa-router";
  import * as path from "../path.js";
  import { head } from "../stores.js";
  import { Icon } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
  import Folder from "./Folder.svelte";

  export let projectId = null;
</script>

<style>
  nav {
    width: var(--project-sidebar-width);
    height: 100%;
    background-color: var(--color-almostwhite);
    display: flex;
    position: fixed;
    flex-direction: column;
    justify-content: space-between;
    padding: 44px 20px 0 24px;
    overflow-y: auto;
  }

  li {
    margin-bottom: 14px;
  }

  a {
    display: flex;
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

<nav>
  <ul>
    <li>
      <ProjectSidebarItem
        icon={Icon.Home}
        title="Overview"
        href={path.projectOverview({ id: projectId })}
        active={path.active(path.projectOverview({
            id: projectId
          }), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Feed}
        title="Feed"
        href={path.projectFeed({ id: projectId })}
        active={path.active(path.projectFeed({ id: projectId }), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Member}
        title="Members"
        href={path.projectMembers({ id: projectId })}
        active={path.active(path.projectMembers({
            id: projectId
          }), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Fund}
        title="Fund"
        href={path.projectFunds({ id: projectId })}
        active={path.active(path.projectFunds({ id: projectId }), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Source}
        title="Source"
        href={path.projectSource({ id: projectId })}
        active={path.active(path.projectSource({ id: projectId }), $location)}>

        <div class="source-tree">
          <Folder {projectId} prefix={'/'} />
        </div>
      </ProjectSidebarItem>
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Commit}
        title="Commits"
        href={path.projectCommits({ id: projectId })}
        active={path.active(path.projectCommits({
            id: projectId
          }), $location)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Branch}
        title="Branches"
        href={path.projectBranches({ id: projectId })}
        active={path.active(path.projectBranches({
            id: projectId
          }), $location)} />
    </li>
  </ul>
</nav>
