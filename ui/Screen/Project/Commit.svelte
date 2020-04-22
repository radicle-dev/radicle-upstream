<script>
  import { gql } from "apollo-boost";
  import { getClient, query } from "svelte-apollo";
  import { format } from "timeago.js";

  import { showNotification } from "../../store/notification.js";

  import { Title, Flex, Icon } from "../../DesignSystem/Primitive";

  export let params = null;
  const projectId = params.id;
  const commitHash = params.hash;

  const GET_COMMIT = gql`
    query($projectId: ID!, $commitHash: String!) {
      commit(id: $projectId, sha1: $commitHash) {
        author {
          email
          name
        }
        committer {
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
      commit.changeset = {
        summary: {
          additions: 32,
          deletions: 24
        },
        files: [
          {
            path: "core/index.js",
            hunks: [
              {
                expanded: true,
                lines: [
                  { num: [192, 192], type: "", content: "/*" },
                  { num: [193, 193], type: "", content: " * Say hello" },
                  { num: [194, 194], type: "", content: " */" },
                  {
                    num: [195, null],
                    type: "-",
                    content:
                      "Server.prototype.hello = function (req, contentType) {"
                  },
                  {
                    num: [null, 195],
                    type: "+",
                    content:
                      "Server.prototype.hello = function (req, contentType) {"
                  },
                  {
                    num: [196, 196],
                    type: "",
                    content: "    var enable = this.options.gzip;"
                  },
                  {
                    num: [197, 197],
                    type: "",
                    content: "    if (enable && (typeof enable === 'boolean' ||"
                  }
                ]
              }
            ]
          },
          {
            path: "core/server.js",
            hunks: [
              {
                expanded: true,
                lines: [
                  {
                    num: [192, 192],
                    type: "",
                    content:
                      "/* Check if we should consider sending a gzip version of the file based on the"
                  },
                  {
                    num: [193, 193],
                    type: "",
                    content:
                      " * file content type and client's Accept-Encoding header value."
                  },
                  { num: [194, 194], type: "", content: " */" },
                  {
                    num: [195, null],
                    type: "-",
                    content:
                      "Server.prototype.ok = function (req, contentType) {"
                  },
                  {
                    num: [null, 195],
                    type: "+",
                    content:
                      "Server.prototype.gzipOk = function (req, contentType) {"
                  },
                  {
                    num: [196, 196],
                    type: "",
                    content: "    var enable = this.options.gzip;"
                  },
                  { num: [197, 197], type: "", content: "    if (enable &&" },
                  {
                    num: [198, 198],
                    type: "",
                    content: "        (typeof enable === 'boolean' ||"
                  }
                ]
              },
              {
                expanded: false,
                header:
                  "@@ -206,20 +206,17 @@ Server.prototype.gzipOk = function(req, contentType) {",
                lines: [
                  {
                    num: [199, 199],
                    type: "",
                    content:
                      "            (contentType && (enable instanceof RegExp) && enable.test(contentType)))) {"
                  },
                  {
                    num: [200, 200],
                    type: "",
                    content:
                      "        var acceptEncoding = req.headers['accept-encoding'];"
                  },
                  {
                    num: [201, 201],
                    type: "",
                    content:
                      "        return acceptEncoding && acceptEncoding.indexOf('gzip') >= 0;"
                  },
                  { num: [202, 202], type: "", content: "    }" },
                  { num: [203, 203], type: "", content: "    return false;" },
                  { num: [204, 204], type: "", content: "}" },
                  { num: [205, 205], type: "", content: "" }
                ]
              },
              {
                expanded: true,
                lines: [
                  {
                    num: [206, null],
                    type: "-",
                    content:
                      "Server.prototype.respond = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {"
                  },
                  {
                    num: [null, 206],
                    type: "+",
                    content:
                      "/* Send a gzipped version of the file if the options and the client indicate gzip is enabled and"
                  },
                  {
                    num: [null, 207],
                    type: "+",
                    content:
                      " * we find a .gz file mathing the static resource requested."
                  },
                  { num: [null, 208], type: "+", content: " */" },
                  {
                    num: [null, 209],
                    type: "+",
                    content:
                      "Server.prototype.respondGzip = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {"
                  },
                  {
                    num: [207, 210],
                    type: "",
                    content: "    var that = this;"
                  },
                  {
                    num: [208, 211],
                    type: "",
                    content:
                      "    if (files.length == 1 && this.gzipOk(req, contentType)) {"
                  },
                  {
                    num: [209, 212],
                    type: "",
                    content: "        var gzFile = files[0] + '.gz';"
                  },
                  {
                    num: [210, 213],
                    type: "",
                    content: "        fs.stat(gzFile, function (e, gzStat) {"
                  },
                  {
                    num: [211, 214],
                    type: "",
                    content: "            if (!e && gzStat.isFile()) {"
                  },
                  {
                    num: [212, 215],
                    type: "",
                    content: "                var vary = _headers['Vary'];"
                  },
                  {
                    num: [213, null],
                    type: "-",
                    content:
                      "                _headers['Vary'] = (vary && vary != 'Accept-Encoding'?vary+', ':'')+'Accept-Encoding';"
                  },
                  {
                    num: [null, 216],
                    type: "+",
                    content:
                      "                _headers['Vary'] = (vary && vary != 'Accept-Encoding' ? vary + ', ' : '') + 'Accept-Encoding';"
                  },
                  {
                    num: [214, 217],
                    type: "",
                    content:
                      "                _headers['Content-Encoding'] = 'gzip';"
                  },
                  {
                    num: [215, 218],
                    type: "",
                    content: "                stat.size = gzStat.size;"
                  },
                  {
                    num: [216, 219],
                    type: "",
                    content: "                files = [gzFile];"
                  },
                  {
                    num: [217, null],
                    type: "-",
                    content: "            } else {"
                  },
                  {
                    num: [218, null],
                    type: "-",
                    content:
                      "                console.log('gzip file not found or error finding it', gzFile, String(e), stat.isFile());"
                  },
                  { num: [219, 220], type: "", content: "            }" },
                  {
                    num: [220, 221],
                    type: "",
                    content:
                      "            that.respondNoGzip(pathname, status, contentType, _headers, files, stat, req, res, finish);"
                  },
                  { num: [221, 222], type: "", content: "        });" }
                ]
              }
            ]
          }
        ]
      };

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
  .hash {
    font-family: var(--typeface-mono-regular);
  }

  .changeset-summary {
    margin-top: 2rem;
    margin-bottom: 1.5rem;
    margin-left: 1.5rem;
  }
  .changeset-summary .additions {
    color: var(--color-positive);
    font-weight: 600;
  }
  .changeset-summary .deletions {
    color: var(--color-negative);
    font-weight: 600;
  }

  /* TODO(cloudhead): Reconcile with `FileSource`? */
  .changeset-file {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 3px;
    min-width: var(--content-min-width);
    margin-bottom: 2rem;
  }
  .changeset-file header {
    height: 3rem;
    display: flex;
    align-items: center;
    background: none;
    border-bottom: 1px solid var(--color-foreground-level-3);
    border-radius: 0;
    padding: 0.75rem;
  }
  .changeset-file-path {
    font-weight: 600;
    margin-left: 0.5rem;
  }
  .changeset-file main {
    overflow-x: auto;
  }

  table.diff {
    table-layout: fixed;
    border-collapse: collapse;
  }

  tr.diff-line[data-type="+"] > * {
    background: var(--color-positive-level-1);
  }
  tr.diff-line[data-type="-"] > * {
    background: var(--color-negative-level-1);
  }
  td.diff-line-number {
    text-align: center;
    user-select: none;
    line-height: 150%;
    padding: 0 0.5rem;
  }
  td.diff-line-content {
    white-space: pre;
  }
  td.diff-line-type {
    color: var(--color-foreground-level-6);
    user-select: none;
    padding: 0 0.5rem;
    text-align: center;
  }
  td.diff-expand-action {
    text-align: center;
    user-select: none;
  }
  td.diff-expand-header {
    padding-left: 0.5rem;
    user-select: none;
  }

  td.diff-expand-header,
  td.diff-expand-action,
  td.diff-line-number {
    color: var(--color-foreground-level-5);
    background-color: var(--color-foreground-level-1);
  }

  td.diff-expand-header,
  td.diff-expand-action,
  td.diff-line-type,
  td.diff-line-content,
  td.diff-line-number {
    font-family: var(--typeface-mono-regular);
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
          <span class="hash">{commit.sha1}</span>
        </p>
      </div>
    </Flex>
  </header>
  <main>
    <div class="changeset-summary">
      {commit.changeset.files.length} file(s) changed with
      <span class="additions">
        {commit.changeset.summary.additions} addition(s)
      </span>
      and
      <span class="deletions">
        {commit.changeset.summary.deletions} deletion(s)
      </span>
    </div>
    {#each commit.changeset.files as file}
      <article class="changeset-file">
        <header>
          <Icon.File />
          <span class="changeset-file-path">{file.path}</span>
        </header>
        <main>
          <table class="diff">
            {#each file.hunks as hunk}
              {#if hunk.expanded}
                {#each hunk.lines as line}
                  <tr class="diff-line" data-expanded data-type={line.type}>
                    <td class="diff-line-number">{line.num[0] || ''}</td>
                    <td class="diff-line-number">{line.num[1] || ''}</td>
                    <td class="diff-line-type">{line.type}</td>
                    <td class="diff-line-content">{line.content}</td>
                  </tr>
                {/each}
              {:else}
                <tr class="diff-line">
                  <td colspan="2" class="diff-expand-action">...</td>
                  <td colspan="2" class="diff-expand-header">{hunk.header}</td>
                </tr>
              {/if}
            {/each}
          </table>
        </main>
      </article>
    {/each}
  </main>
{/await}
