<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as router from "ui/src/router";

  import type { Project } from "ui/src/project";
  import { fetchProjects, projects as store } from "../../src/userProfile";

  import { Error, ProjectList, Remote } from "ui/DesignSystem";

  export let urn: string;

  const select = ({ detail: project }: { detail: Project }) => {
    router.push({
      type: "project",
      urn: project.urn,
      activeView: { type: "files" },
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
