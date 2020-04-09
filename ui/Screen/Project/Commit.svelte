<script>
  import { gql } from "apollo-boost";
  import { link } from "svelte-spa-router";
  import { getClient, query } from "svelte-apollo";
  import { format } from "timeago.js";

  import { showNotification } from "../../store/notification.js";
  import * as path from "../../lib/path.js";

  import { Title, Flex, Icon } from "../../DesignSystem/Primitive";

  export let params = null;
  const projectId = params.id;
  const commitHash = params.hash;

  const GET_COMMIT = gql`
    query($projectId: ID!, $commitHash: String!) {
      commit(id: $projectId, sha1: $commitHash) {
        author {
          avatar
          email
          name
        }
        committer {
          avatar
          email
          name
        }
        committerTime
        description
        sha1
        summary
      }
    }
  `;

  async function fetchCommit() {
    try {
      const response = await query(getClient(), {
        query: GET_COMMIT,
        variables: {
          projectId: projectId,
          commitHash: commitHash
        }
      });
      const result = await response.result();
      const commit = result.data.commit;

      // TODO(cloudhead): Fetch branch from backend.
      commit.branch = "master";

      return commit;
    } catch (error) {
      showNotification({
        text: "Could not fetch commit",
        level: "error"
      });
    }
  }
</script>

<style>
  header {
    background: var(--color-foreground-level-1);
    border-radius: 4px;
    padding: 1.5rem;
  }
  .description {
    font-family: var(--typeface-mono-regular);
  }
  .field {
    color: var(--color-foreground-level-6);
    margin-bottom: 0.5rem;
  }
  .field:last-child {
    margin-bottom: 0;
  }
  .email {
    font-family: var(--typeface-mono-regular);
  }
  .branch {
    margin: 0 0.5rem;
    font-weight: bold;
    color: var(--color-foreground-level-6);
  }
  .author {
    font-weight: bold;
    color: var(--color-foreground);
  }

  /* TODO(cloudhead): These should be global */
  a {
    color: var(--color-secondary);
  }
  hr {
    border: 0;
    border-top: 1px solid var(--color-foreground-level-3);
    margin: 1rem 0 1.5rem 0;
  }
</style>

{#await fetchCommit() then commit}
  <header>
    <Flex style="align-items: flex-start">
      <div slot="left">
        <Title variant="large" style="margin-bottom: 1rem">
          {commit.summary}
        </Title>
      </div>
      <div slot="right">
        <span class="field">
          <!-- NOTE(cloudhead): These awful margin hacks are here because
          there is a bug in prettier that breaks our HTML if we try to format
          it differently. -->
          <span style="margin-right: -1ch">Committed to</span>
          <span class="branch">
            <Icon.Branch
              style="vertical-align: bottom; fill:
              var(--color-foreground-level-6)" />
            <span style="margin-left: -0.5ch">{commit.branch}</span>
          </span>
          <span style="margin-left: -0.5ch">
            {format(commit.committerTime)}
          </span>
        </span>
      </div>
    </Flex>
    <pre class="description" style="margin-bottom: 1rem">
      {commit.description}
    </pre>
    <hr />
    <Flex style="align-items: flex-end">
      <div slot="left">
        <p class="field">
          Authored by
          <span class="author">{commit.author.name}</span>
          <span class="email">&lt;{commit.author.email}&gt;</span>
        </p>
        {#if commit.committer.email != commit.author.email}
          <p class="field">
            Committed by
            <span class="author">{commit.committer.name}</span>
            <span class="email">&lt;{commit.committer.email}&gt;</span>
          </p>
        {/if}
      </div>
      <div slot="right">
        <!-- TODO(cloudhead): Commit parents when dealing with merge commit -->
        <p class="field">
          Commit
          <a href={path.projectCommit(projectId, commit.sha1)} use:link>
            {commit.sha1}
          </a>
        </p>
      </div>
    </Flex>
  </header>
{/await}
