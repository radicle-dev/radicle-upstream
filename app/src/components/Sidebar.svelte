<script>
  import { Icon } from "../DesignSystem";
  import SidebarItem from "./SidebarItem.svelte";
  import SidebarAvatar from "./SidebarAvatar.svelte";
  import * as path from "../path.js";
</script>

<style>
  div {
    width: 68px;
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

<div>
  <nav>
    <ul>
      <li>
        <SidebarItem
          icon={Icon.Search}
          title="Search"
          href={path.makeHref(path.SEARCH)}
          active={path.isActive(path.SEARCH)} />
      </li>
      <li>
        <SidebarItem
          icon={Icon.Feed}
          title="Feed"
          href={path.makeHref(path.FEED)}
          active={path.isActive(path.FEED)} />
      </li>
      <li>
        <SidebarItem
          icon={Icon.Projects}
          title="Projects"
          href={path.makeHref(path.PROJECTS)}
          active={path.isActive(path.PROJECTS)} />
      </li>
    </ul>
  </nav>

  <nav>
    <ul>
      <li>
        <SidebarItem
          icon={Icon.Plus}
          title="Create new project"
          href={path.makeHref(path.DESIGN_SYSTEM)}
          active={path.isActive(path.DESIGN_SYSTEM)} />
      </li>
      <li>
        <SidebarItem
          icon={Icon.Fund}
          title="Fund"
          href={path.makeHref(path.WALLET)}
          active={path.isActive(path.WALLET)} />
      </li>
      <li>
        <SidebarAvatar
          image="https://avatars2.githubusercontent.com/u/2326909?s=400&v=4"
          title="Profile"
          href={path.makeHref(path.PROFILE)}
          active={path.isActive(path.PROFILE)} />
      </li>
    </ul>
  </nav>
</div>
