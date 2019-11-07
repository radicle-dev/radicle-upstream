<script>
  import { Icon } from "../DesignSystem";
  import SidebarItem from "./SidebarItem.svelte";
  import SidebarAvatar from "./SidebarAvatar.svelte";
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

  :global(.Sidebar li .show-on-hover) {
    opacity: 0;
    display: none;
  }

  :global(.Sidebar li:hover svg) {
    fill: var(--color-purple);
  }

  :global(.Sidebar li:hover .show-on-hover) {
    opacity: 1;
    display: inline-block;
  }
</style>

<div class="Sidebar">
  <nav>
    <ul>
      <li>
        <SidebarItem icon={Icon.Search} href="/search" title="Search" />
      </li>
      <li>
        <SidebarItem icon={Icon.Feed} href="/feed" title="Feed" />
      </li>
      <li>
        <SidebarItem icon={Icon.Projects} href="/projects" title="Projects" />
      </li>
    </ul>
  </nav>

  <nav>
    <ul>
      <li>
        <SidebarItem
          icon={Icon.Plus}
          href="/design-system"
          title="Create new project" />
      </li>
      <li>
        <SidebarItem icon={Icon.Fund} href="/wallet" title="Fund" />
      </li>
      <li>
        <SidebarAvatar
          image="https://avatars2.githubusercontent.com/u/2326909?s=400&v=4"
          href="/profile"
          title="Profile" />
      </li>
    </ul>
  </nav>
</div>
