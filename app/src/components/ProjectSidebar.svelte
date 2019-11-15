<script>
  import * as path from "../path.js";
  import { head } from "../stores.js";
  import { Icon } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
  import Folder from "./Folder.svelte";

  export let projectId = null;
</script>

<style>
  nav {
    width: 250px;
    height: 100%;
    background-color: var(--color-almostwhite);
    display: flex;
    position: fixed;
    left: 68px;
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
        active={path.isActive(path.projectOverview({ id: projectId }))} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Feed}
        title="Feed"
        href={path.projectFeed({ id: projectId })}
        active={path.isActive(path.projectFeed({ id: projectId }))} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Member}
        title="Members"
        href={path.projectMembers({ id: projectId })}
        active={path.isActive(path.projectMembers({ id: projectId }))} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Fund}
        title="Fund"
        href={path.projectFunds({ id: projectId })}
        active={path.isActive(path.projectFunds({ id: projectId }))} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Source}
        title="Source"
        href={path.projectSource({ id: projectId })}
        active={path.isActive(path.projectSource({ id: projectId }))}>

        <div class="source-tree">
          <Folder {projectId} prefix={'/'} {$head} />
        </div>
      </ProjectSidebarItem>
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Commit}
        title="Commits"
        href={path.projectCommits({ id: projectId })}
        active={path.isActive(path.projectCommits({ id: projectId }))} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Branch}
        title="Branches"
        href={path.projectBranches({ id: projectId })}
        active={path.isActive(path.projectBranches({ id: projectId }))} />
    </li>
  </ul>
</nav>
