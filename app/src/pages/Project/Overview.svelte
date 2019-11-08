<script>
  import Layout from "../../components/Layout.svelte";
  import ProjectSidebar from "../../components/ProjectSidebar.svelte";
  import { Button, Title, Text } from "../../DesignSystem";
  export let params = {};

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: ProjectId!) {
      project(id: $id) {
        name
        description
        imgUrl
        members {
          keyName
          avatarUrl
        }
      }
    }
  `;

  const client = getClient();
  const project = query(client, {
    query: GET_PROJECT,
    variables: { id: params.id }
  });
</script>

<style>
  .header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
  .info {
    display: grid;
    grid-template-columns: 64px auto auto;
    grid-column-gap: 24px;
    padding-bottom: 16px;
  }
  .avatar {
    width: 64px;
    height: 64px;
    border-radius: 2px;
  }

  hr {
    margin-top: 32px;
    border-top: var(--color-gray);
  }
</style>

<Layout>
  <div slot="nestedSidebar">
    <ProjectSidebar projectId={params.id} />
  </div>

  {#await $project}
    <h1>Loading project...</h1>
  {:then result}
    <div class="header">
      <div>
        <div class="info">
          <img class="avatar" src={result.data.project.imgUrl} alt="" />
          <div>
            <Title.Big>{result.data.project.name}</Title.Big>
            <Text.Regular style="color: var(--color-gray)">
              {result.data.id}
            </Text.Regular>
          </div>
        </div>
        <Text.Regular style="color: var(--color-dark-gray)">
          {result.data.project.description}
        </Text.Regular>
      </div>
      <Button size="small" variant="vanilla">Clone</Button>
    </div>
    <hr />
  {:catch error}
    <p>ERROR: {error}</p>
  {/await}
</Layout>
