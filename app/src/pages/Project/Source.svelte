<script>
  import Layout from "../../components/Layout.svelte";
  import ProjectSidebar from "../../components/ProjectSidebar.svelte";

  export let params = null;
</script>

<Layout>
  <div slot="nestedSidebar">
    <ProjectSidebar projectId={params.id} />
  </div>
  Source
</Layout>
