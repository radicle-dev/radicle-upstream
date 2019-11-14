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
        href={path.makeHref(path.PROJECT_OVERVIEW, { id: projectId })}
        active={path.isActive(path.PROJECT_OVERVIEW)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Feed}
        title="Feed"
        href={path.makeHref(path.PROJECT_FEED, { id: projectId })}
        active={path.isActive(path.PROJECT_FEED)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Member}
        title="Members"
        href={path.makeHref(path.PROJECT_MEMBERS, { id: projectId })}
        active={path.isActive(path.PROJECT_MEMBERS)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Fund}
        title="Fund"
        href={path.makeHref(path.PROJECT_FUNDS, { id: projectId })}
        active={path.isActive(path.PROJECT_FUNDS)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Source}
        title="Source"
        href={path.makeHref(path.PROJECT_SOURCE, { id: projectId })}
        active={path.isActive(path.PROJECT_SOURCE)}>

        <div class="source-tree">
          <Folder {projectId} prefix={'./'} head={$head} />
        </div>
      </ProjectSidebarItem>
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Commit}
        title="Commits"
        href={path.makeHref(path.PROJECT_COMMITS, { id: projectId })}
        active={path.isActive(path.PROJECT_COMMITS)} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Branch}
        title="Branches"
        href={path.makeHref(path.PROJECT_BRANCHES, { id: projectId })}
        active={path.isActive(path.PROJECT_BRANCHES)} />
    </li>
  </ul>
</nav>
