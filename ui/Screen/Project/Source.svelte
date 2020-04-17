<script>
  import { getContext } from "svelte";

  import { Code, Text, Title } from "../../DesignSystem/Primitive";
  import { SourceBrowser } from "../../DesignSystem/Component";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";

  const GET_PROJECT = gql`
    query Query($id: ID!) {
      project(id: $projectId) {
        metadata {
          name
          description
        }
      }
    }
  `;

  const client = getClient();
  const projectId = getContext("projectId");
  const project = query(client, {
    query: GET_PROJECT,
    variables: { projectId: projectId }
  });
</script>

<style>
  .header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
  }
  .project-id {
    color: var(--color-foreground-level-5);
    margin-top: 0.5rem;
  }
  .description {
    margin-top: 1rem;
  }
</style>

<div class="header">
  {#await $project then result}
    <Title variant="big">{result.data.project.metadata.name}</Title>
    <div class="project-id">
      <Code>%{projectId}</Code>
    </div>
    <div class="description">
      <Text>{result.data.project.metadata.description}</Text>
    </div>
  {/await}
</div>

<SourceBrowser />
