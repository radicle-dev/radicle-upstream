<script lang="typescript">
  import Router, { push } from "svelte-spa-router";

  import * as path from "../../src/path";
  import type { Urn } from "../../src/urn";

  import Code from "./Source/Code.svelte";
  import Commit from "./Source/Commit.svelte";
  import Commits from "./Source/Commits.svelte";

  export let projectUrn: Urn;

  const routes = {
    "/projects/:urn/source/code": Code,
    "/projects/:urn/source/commit/:hash": Commit,
    "/projects/:urn/source/commits": Commits,
  };

  push(path.projectSourceCode(projectUrn));
</script>

<Router {routes} />
