<script>
  import { Icon, Text } from "../DesignSystem";
  import ProjectSidebarItem from "./ProjectSidebarItem.svelte";
  import Folder from "./Folder.svelte";
  import { link, location } from "svelte-spa-router";

  import {
    ProjectOverview,
    ProjectFeed,
    ProjectMembers,
    ProjectFunds,
    ProjectSource,
    ProjectCommits,
    ProjectBranches
  } from "../routes.js";

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
      <ProjectSidebarItem {projectId} page={ProjectOverview} />
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectFeed} />
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectMembers} />
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectFunds} />
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectSource}>
        <div class="source-tree">
          <Folder {projectId} prefix={'./'} head="master" />
        </div>
      </ProjectSidebarItem>
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectCommits} />
    </li>

    <li>
      <ProjectSidebarItem {projectId} page={ProjectBranches} />
    </li>
  </ul>
</nav>
