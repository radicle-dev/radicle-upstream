<script>
  import { Icon, Text } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
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
    z-index: 900;
    overflow-y: scroll;
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

  nav :global(li:hover svg) {
    fill: var(--color-purple);
  }
</style>

<nav>
  <ul>
    <li>
      <a href="/projects/{projectId}/overview" use:link>
        <ProjectSidebarItem
          icon={Icon.Home}
          title="Overview"
          active={$location.endsWith('overview')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/feed" use:link>
        <ProjectSidebarItem
          icon={Icon.Feed}
          title="Feed"
          active={$location.endsWith('feed')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/members" use:link>
        <ProjectSidebarItem
          icon={Icon.Member}
          title="Members"
          active={$location.endsWith('members')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/funds" use:link>
        <ProjectSidebarItem
          icon={Icon.Fund}
          title="Fund"
          active={$location.endsWith('funds')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/source" use:link>
        <ProjectSidebarItem
          icon={Icon.Source}
          title="Source"
          active={$location.endsWith('source')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/commits" use:link>
        <ProjectSidebarItem
          icon={Icon.Commit}
          title="Commits"
          active={$location.endsWith('commits')} />
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/branches" use:link>
        <ProjectSidebarItem
          icon={Icon.Branch}
          title="Branches"
          active={$location.endsWith('branches')} />
      </a>
    </li>
  </ul>
</nav>
