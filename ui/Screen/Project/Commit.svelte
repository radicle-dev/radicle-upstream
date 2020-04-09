<script>
  import { Title, Flex } from "../../DesignSystem/Primitive";
  import { Icon } from "../../DesignSystem/Primitive";
  import { gql } from "apollo-boost";
  import { link } from "svelte-spa-router";
  import { showNotification } from "../../store/notification.js";
  import { getClient, query } from "svelte-apollo";
  import { format } from "timeago.js";
  import * as path from "../../lib/path.js";

  export let params = null;
  const projectId = params.id;
  const commitHash = params.hash;

  const QUERY = gql`
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
        message
        sha1
        summary
      }
    }
  `;

  async function fetchCommit() {
    try {
      const response = await query(getClient(), {
        query: QUERY,
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
  .message {
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
          Committed to
          <span class="branch">
            <Icon.Branch
              color="foreground-level-6"
              style="vertical-align: bottom" />
            {commit.branch}
          </span>
          {format(commit.committerTime)}
        </span>
      </div>
    </Flex>
    <pre class="message" style="margin-bottom: 1rem">{commit.message}</pre>
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
