<script>
  import { location } from "svelte-spa-router";
  import { Icon } from "../Primitives";

  import Item from "./Sidebar/Item.svelte";
  import Avatar from "./Sidebar/Avatar.svelte";

  import * as path from "../../lib/path.js";
</script>

<style>
  div {
    width: var(--sidebar-width);
    height: 100%;
    background-color: #eeeeef;
    display: flex;
    position: fixed;
    flex-direction: column;
    justify-content: space-between;
    padding: 18px;
    z-index: 1000;
  }

  nav ul {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  li {
    width: 32px;
    height: 32px;
    margin-bottom: 18px;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  div :global(li .show-on-hover) {
    opacity: 0;
    display: none;
  }

  div :global(li:hover svg) {
    fill: var(--color-purple);
  }

  div :global(li:hover .show-on-hover) {
    opacity: 1;
    display: inline-block;
  }
</style>

<div data-cy="sidebar">
  <nav>
    <ul>
      <li>
        <Item
          icon={Icon.Search}
          title="Search"
          dataCy="search"
          href={path.search()}
          active={path.active(path.search(), $location)} />
      </li>
      <li>
        <Item
          icon={Icon.Feed}
          title="Feed"
          dataCy="feed"
          href={path.feed()}
          active={path.active(path.feed(), $location)} />
      </li>
      <li>
        <Item
          icon={Icon.Projects}
          title="Projects"
          dataCy="projects"
          href={path.projects()}
          active={path.active(path.projects(), $location)} />
      </li>
    </ul>
  </nav>

  <nav>
    <ul>
      <li>
        <Item
          icon={Icon.Plus}
          title="Create new project"
          dataCy="new-project"
          href={path.createProject()}
          active={path.active(path.createProject(), $location)} />
      </li>
      <li>
        <Item
          icon={Icon.Fund}
          title="Fund"
          dataCy="wallet"
          href={path.wallet()}
          active={path.active(path.wallet(), $location)} />
      </li>
      <li>
        <Avatar
          image="https://avatars2.githubusercontent.com/u/2326909?s=400&v=4"
          title="Profile"
          dataCy="profile"
          href={path.profile()}
          active={path.active(path.profile(), $location)} />
      </li>
    </ul>
  </nav>
</div>
