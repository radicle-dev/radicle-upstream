<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { Project } from "ui/src/project";

  import { fetchProjects, projects as store } from "ui/src/userProfile";
  import * as router from "ui/src/router";

  import Error from "ui/App/ProfileScreen/Error.svelte";
  import ProjectList from "ui/App/ProfileScreen/ProjectList.svelte";
  import Remote from "ui/App/Remote.svelte";

  export let urn: string;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      params: {
        urn: project.urn,
        activeView: { type: "files" },
      },
    });
  };

  fetchProjects(urn);
</script>

<Remote {store} let:data={projects}>
  <ProjectList {projects} userUrn={urn} on:select={select} />

  <div slot="error" let:error>
    <Error message={error.message} />
  </div>
</Remote>
