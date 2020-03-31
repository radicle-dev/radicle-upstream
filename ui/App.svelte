<script>
  import Router from "svelte-spa-router";
  import { initializeHotkeys } from "./lib/hotkeys.js";

  import CreateProject from "./Screen/CreateProject.svelte";
  import DesignSystemGuide from "./Screen/DesignSystemGuide.svelte";
  import Help from "./Screen/Help.svelte";
  import Network from "./Screen/Network.svelte";
  import NotFound from "./Screen/NotFound.svelte";
  import Project from "./Screen/Project.svelte";
  import Projects from "./Screen/Projects.svelte";
  import RegisterProject from "./Screen/RegisterProject.svelte";
  import UserRegistration from "./Screen/UserRegistration.svelte";
  import Search from "./Screen/Search.svelte";
  import CreateIdentity from "./Screen/IdentityCreation.svelte";

  initializeHotkeys();

  const routes = {
    "/identity/new": CreateIdentity,
    "/search": Search,
    "/network": Network,
    "/projects": Projects,
    "/projects/new": CreateProject,
    "/projects/:id/register": RegisterProject,
    "/projects/:id/*": Project,
    "/design-system-guide": DesignSystemGuide,
    "/help": Help,
    "/user-registration": UserRegistration,
    "*": NotFound
  };
</script>

<Router {routes} />
