<script lang="typescript">
  import { routeStore } from "ui/src/router";

  import DesignSystemGuide from "ui/Screen/DesignSystemGuide.svelte";
  import Lock from "ui/Screen/Lock.svelte";
  import Onboarding from "ui/Screen/Onboarding.svelte";
  import Profile from "ui/Screen/Profile.svelte";
  import Project from "ui/Screen/Project.svelte";
  import Settings from "ui/Screen/Settings.svelte";

</script>

{#if $routeStore.type === 'designSystemGuide'}
  <DesignSystemGuide />
{:else if $routeStore.type === 'lock'}
  <Lock />
{:else if $routeStore.type === 'onboarding'}
  <Onboarding />
{:else if $routeStore.type === 'profile'}
  <Profile activeTab={$routeStore.activeTab} />
{:else if $routeStore.type === 'project'}
  <Project
    activeTab={$routeStore.activeTab}
    urn={$routeStore.urn}
    commitHash={$routeStore.commitHash} />
{:else if $routeStore.type === 'settings'}
  <Settings />
{/if}
