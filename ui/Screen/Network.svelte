<script>
  import { Title } from "../DesignSystem/Primitive";
  import { Flex } from "../DesignSystem/Primitive";
  import { ProjectCard, SidebarLayout } from "../DesignSystem/Component";
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECTS = gql`
    query Query {
      listRegistryProjects
    }
  `;

  const client = getClient();
  const projects = query(client, { query: GET_PROJECTS });
  projects.refetch();
</script>

<SidebarLayout dataCy="page">
  <Title variant="big">Network</Title>

  <Flex align="left">
    {#await $projects then result}
      <ul>
        {#each result.data.listRegistryProjects as project}
          <li class="project-card">
            <ProjectCard title={project} isRegistered={true} />
          </li>
        {/each}
      </ul>
    {/await}
  </Flex>
</SidebarLayout>
