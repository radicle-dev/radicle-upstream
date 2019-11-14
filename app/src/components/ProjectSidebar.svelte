<script>
  import { head } from "../stores.js";
  import { Icon, Text } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
  import Folder from "./Folder.svelte";
  import { link, location } from "svelte-spa-router";

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
        {projectId}
        href={`/projects/${projectId}/overview`}
        active={$location.endsWith('overview')} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Feed}
        title="Feed"
        {projectId}
        href={`/projects/${projectId}/feed`}
        active={$location.endsWith('feed')} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Member}
        title="Members"
        {projectId}
        href={`/projects/${projectId}/members`}
        active={$location.endsWith('members')} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Fund}
        title="Fund"
        {projectId}
        href={`/projects/${projectId}/funds`}
        active={$location.endsWith('funds')} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Source}
        title="Source"
        {projectId}
        href={`/projects/${projectId}/source`}
        active={$location.endsWith('source')}>

        <div class="source-tree">
          <Folder {projectId} prefix={'./'} head={$head} />
        </div>
      </ProjectSidebarItem>
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Commit}
        title="Commits"
        {projectId}
        href={`/projects/${projectId}/commits`}
        active={$location.endsWith('commits')} />
    </li>

    <li>
      <ProjectSidebarItem
        icon={Icon.Branch}
        title="Branches"
        {projectId}
        href={`/projects/${projectId}/branches`}
        active={$location.endsWith('branches')} />
    </li>
  </ul>
</nav>
