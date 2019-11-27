<script>
  import Layout from "../../components/Layout.svelte";
  import ProjectSidebar from "../../components/ProjectSidebar.svelte";

  export let params = { name: "", domain: "" };
  let id = { name: params.name, domain: params.domain };
</script>

<Layout>
  <div slot="nestedSidebar">
    <ProjectSidebar projectId={id} />
  </div>
  Members
</Layout>
