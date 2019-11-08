<script>
  import { Icon } from "../DesignSystem";
  import SidebarItem from "./SidebarItem.svelte";
  import SidebarAvatar from "./SidebarAvatar.svelte";

  import {
    Search,
    Feed,
    Projects,
    DesignSystem,
    Wallet,
    Profile
  } from "../routes";
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
        <SidebarItem page={Search} />
      </li>
      <li>
        <SidebarItem page={Feed} />
      </li>
      <li>
        <SidebarItem page={Projects} />
      </li>
    </ul>
  </nav>

  <nav>
    <ul>
      <li>
        <SidebarItem page={DesignSystem} />
      </li>
      <li>
        <SidebarItem page={Wallet} />
      </li>
      <li>
        <SidebarAvatar page={Profile} />
      </li>
    </ul>
  </nav>
</div>
