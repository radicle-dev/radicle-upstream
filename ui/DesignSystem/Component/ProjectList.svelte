<script>
  import { Flex, Text, Button } from "../Primitive";

  import ProjectCard from "./ProjectCard.svelte";
  import Placeholder from "./Placeholder.svelte";

  import { projectNameStore } from "../../store/project.js";
  import * as path from "../../lib/path.js";

  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { push } from "svelte-spa-router";

  const GET_PROJECTS = gql`
    query Query {
      projects {
        id
        metadata {
          defaultBranch
          description
          name
        }
        registered {
          ... on OrgRegistration {
            orgId
          }
          ... on UserRegistration {
            userId
          }
        }
        stats {
          branches
          commits
          contributors
        }
      }
    }
  `;

  const client = getClient();
  const projects = query(client, { query: GET_PROJECTS });
  projects.refetch();
</script>

<style>
  ul {
    min-width: 500px;
  }

  li {
    display: flex;
    width: 100%;
    height: 96px;
    flex: 1;
    border-bottom: 1px solid var(--color-foreground-level-3);
    cursor: pointer;
    padding: 22px 15px 26px 12px;
  }

  li:hover {
    background-color: var(--color-foreground-level-1);
  }

  li:last-child {
    border-bottom: 0;
  }

  .wrapper {
    margin-top: 156px;
    display: flex;
    justify-content: center;
  }

  .create-project {
    text-align: center;
    width: 480px;
  }
</style>

{#await $projects}
  <Text>Loading projects...</Text>
{:then result}
  {#if result.data.projects.length > 0}
    <ul>
      {#each result.data.projects as project}
        <li
          on:click={() => {
            projectNameStore.set(project.metadata.name);
            push(path.projectSource(project.id));
          }}
          class="project-card">
          <ProjectCard
            projectId={project.id}
            title={project.metadata.name}
            description={project.metadata.description}
            isRegistered={project.registered}
            commitCount={project.stats.commits}
            branchCount={project.stats.branches}
            memberCount={project.stats.contributors} />
        </li>
      {/each}
    </ul>
  {:else}
    <div class="wrapper">
      <div class="create-project">
        <Placeholder style="width: 420px; height: 217px;" />
        <Text style="color: var(--color-foreground-level-5)">
          There's nothing here yet, get started by creating your first project
          or register your identity on the Registry.
        </Text>
        <Flex align="center" style="margin-top: 27px;">
          <Button
            variant="vanilla"
            style="margin-right: 16px;"
            on:click={() => {
              push(path.createProject());
            }}>
            Start a new project
          </Button>
          <Button
            variant="vanilla"
            on:click={() => {
              push(path.registerUser());
            }}>
            Register radicleID
          </Button>
        </Flex>
      </div>
    </div>
  {/if}
{:catch error}
  <p>ERROR: {error}</p>
{/await}
