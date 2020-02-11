<script>
  import { getContext } from "svelte";
  import { location } from "svelte-spa-router";

  import * as path from "../../lib/path.js";

  import { Icon } from "../Primitives";

  import ProjectTopbarMenuItem from "./ProjectTopbarMenuItem.svelte";

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

<nav data-cy="project-topbar-menu">
  <ul>

    <li>
      <ProjectTopbarMenuItem
        icon={Icon.Source}
        title="Source"
        href={path.projectSource(projectId)}
        active={path.active(path.projectSource(projectId), $location, true)} />
    </li>

    <li>
      <ProjectTopbarMenuItem
        icon={Icon.Feed}
        title="Feed"
        href={path.projectFeed(projectId)}
        active={path.active(path.projectFeed(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarMenuItem
        icon={Icon.Fund}
        title="Fund"
        href={path.projectFunds(projectId)}
        active={path.active(path.projectFunds(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarMenuItem
        icon={Icon.Issues}
        title="Issues"
        href={path.projectIssues(projectId)}
        active={path.active(path.projectIssues(projectId), $location)} />
    </li>

    <li>
      <ProjectTopbarMenuItem
        icon={Icon.Revisions}
        title="Revisions"
        href={path.projectRevisions(projectId)}
        active={path.active(path.projectRevisions(projectId), $location)} />
    </li>
  </ul>
</nav>
