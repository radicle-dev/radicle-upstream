<script>
  import Router from "svelte-spa-router";
  import { link } from "svelte-spa-router";
  import { Icon, Text } from "../DesignSystem";
  import * as path from "../path.js";

  import Overview from "./projectBreadcrumbs/Overview.svelte";
  import Feed from "./projectBreadcrumbs/Feed.svelte";
  import Members from "./projectBreadcrumbs/Members.svelte";
  import Funds from "./projectBreadcrumbs/Funds.svelte";
  import Source from "./projectBreadcrumbs/Source.svelte";
  import Commits from "./projectBreadcrumbs/Commits.svelte";
  import Branches from "./projectBreadcrumbs/Branches.svelte";

  export let style = null;
  export let project = null;

  const routes = {
    "/projects/:id/overview": Overview,
    "/projects/:id/feed": Feed,
    "/projects/:id/members": Members,
    "/projects/:id/funds": Funds,
    "/projects/:id/source": Source,
    "/projects/:id/source/*": Source,
    "/projects/:id/commits": Commits,
    "/projects/:id/branches": Branches
  };
</script>

<style>
  .breadcrumbs {
    display: flex;
    align-items: center;
    background-color: var(--color-white);
    width: 100%;
    padding: 20px 0 20px 24px;
  }

  .breadcrumbs img {
    width: 16px;
    height: 16px;
    margin-right: 6px;
  }

  a {
    display: flex;
    align-items: center;
    border-radius: 2px;
    padding: 2px 8px 2px 8px;
  }

  a:hover {
    background-color: var(--color-almostwhite);
  }
</style>

<div data-cy="breadcrumbs" class="breadcrumbs" {style}>
  <a href={path.projects()} use:link>
    <Icon.Projects style="margin-right: 6px" />
    <Text.Regular>My Projects</Text.Regular>
  </a>

  <Text.Regular>/</Text.Regular>

  <a href={path.projectOverview(project.id)} use:link>
    <img alt="Project Avatar" src={project.metadata.imgUrl} />
    <Text.Regular>{project.metadata.name}</Text.Regular>
  </a>

  <Text.Regular style="margin: 0 6px 0 0">/</Text.Regular>

  <Router {routes} />
</div>
