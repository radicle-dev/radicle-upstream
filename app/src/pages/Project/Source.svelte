<script>
  import { head } from "../../stores.js";
  import Layout from "../../components/Layout.svelte";
  import ProjectSidebar from "../../components/ProjectSidebar.svelte";

  export let params = null;

  let branches = ["master", "code-browsing"];

  let tags = ["v0.0.1"];

  let commits = [
    "4f34a7e27c56983f4082eec7148ff2de8943a4c7",
    "c56499bd12d0b7ab26f01466fe248a62e54cac4f",
    "cfca2129c335f800b7939d64fc50248f5bfffff4"
  ];
</script>

<Layout>
  <div slot="nestedSidebar">
    <ProjectSidebar projectId={params.id} />
  </div>

  <select bind:value={$head}>
    {#each [...branches, ...tags, ...commits] as availableHead}
      <option value={availableHead}>{availableHead}</option>
    {/each}
  </select>
</Layout>
