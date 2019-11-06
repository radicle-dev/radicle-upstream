<script>
  import { Icon, Text } from "../DesignSystem";
  import { link } from "svelte-spa-router";

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

  .icon {
    margin-right: 12px;
    align-self: center;
  }

  .itemWithCarret {
    display: flex;
    width: 100%;
  }
</style>

<nav>
  <ul>
    <li>
      <a href="/projects/{projectId}/overview" use:link>
        <div class="icon">
          <Icon.Home />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">
          Overview
        </Text.Regular>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/feed" use:link>
        <div class="icon">
          <Icon.Feed />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">Feed</Text.Regular>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/members" use:link>
        <div class="icon">
          <Icon.Member />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">
          Members
        </Text.Regular>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/funds" use:link>
        <div class="icon">
          <Icon.Fund />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">Fund</Text.Regular>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/source" use:link>
        <div class="icon">
          <Icon.Source />
        </div>
        <div class="itemWithCarret">
          <Text.Regular style="color: var(--color-darkgray); flex: 1">
            Source
          </Text.Regular>
          <Icon.Carret style="align-self: center; transform: rotate(90deg)" />
        </div>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/commits" use:link>
        <div class="icon">
          <Icon.Commit />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">
          Commits
        </Text.Regular>
      </a>
    </li>

    <li>
      <a href="/projects/{projectId}/branches" use:link>
        <div class="icon">
          <Icon.Branch />
        </div>
        <Text.Regular style="color: var(--color-darkgray)">
          Branches
        </Text.Regular>
      </a>
    </li>
  </ul>
</nav>
