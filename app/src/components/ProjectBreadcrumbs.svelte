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
    "/projects/:domain/:name/overview": Overview,
    "/projects/:domain/:name/feed": Feed,
    "/projects/:domain/:name/members": Members,
    "/projects/:domain/:name/funds": Funds,
    "/projects/:domain/:name/source": Source,
    "/projects/:domain/:name/source/*": Source,
    "/projects/:domain/:name/commits": Commits,
    "/projects/:domain/:name/branches": Branches
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

<div class="breadcrumbs" {style}>
  <a href={path.projects()} use:link>
    <Icon.Projects style="margin-right: 6px" />
    <Text.Regular>My Projects</Text.Regular>
  </a>

  <Text.Regular>/</Text.Regular>

  <a href={path.projectOverview({ id: project.id })} use:link>
    <img alt="Project Avatar" src={project.imgUrl} />
    <Text.Regular>{project.name}</Text.Regular>
  </a>

  <Text.Regular style="margin: 0 6px 0 0">/</Text.Regular>

  <Router {routes} />
</div>
