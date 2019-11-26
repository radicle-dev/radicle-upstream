import { GraphQLServer } from "graphql-yoga";
import { isBinary } from "istextorbinary";
import util from "util";
import path from "path";

const exec = util.promisify(require("child_process").exec);

const typeDefs = `
  type Info {
    mode: String!,
    isDirectory: Boolean!,
    lastCommit: String!,
    size: Int!,
    name: String!
  }

  type Entry {
    path: String,
    info: Info
  }

  type Query {
    tree(projectId: String!, revision: String!, prefix: String!): [Entry]!
    blob(projectId: String!, revision: String!, path: String!): String!
    branches(projectId: String!): [String]!
    tags(projectId: String!): [String]!
  }
`;

const debug = false;

// TODO: implement actual lookup once we have mock data with stable IDs
const projectRepoPathById = _projectId => path.resolve(__dirname, "../../");

const execOptions = projectId => ({
  maxBuffer: 1024 * 1000,
  cwd: projectRepoPathById(projectId)
});

const log = message => {
  debug && console.log(message);
};

async function tree(projectId, revision, prefix) {
  log(`projectId: ${projectId}`);
  log(`revision: ${revision}`);
  log(`prefix: ${prefix}`);

  const command = `git ls-tree --long ${revision} ./${prefix}`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(projectId));

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== "") // throw out empty rows
    .map(row => {
      const [
        mode,
        treeOrBlob,
        lastCommit,
        sizeOrDash,
        nameWithPath
      ] = row.split(/\s+/);

      return {
        path: nameWithPath,
        info: {
          mode: mode,
          isDirectory: treeOrBlob === "tree",
          lastCommit: lastCommit,
          size: sizeOrDash === "-" ? 0 : parseInt(sizeOrDash),
          name: nameWithPath.replace(prefix, "")
        }
      };
    })
    .sort(function(a, b) {
      // sort directories first, then files alphabetically
      if (a.info.isDirectory && !b.info.isDirectory) return -1;
      if (!a.info.isDirectory && b.info.isDirectory) return 1;
      if (a.info.toLowerCase > b.info.toLowerCase) return 1;
    });
}

async function blob(projectId, revision, path) {
  log(`projectId: ${projectId}`);
  log(`revision: ${revision}`);
  log(`path: ${path}`);

  const command = `git show ${revision}:${path}`;
  log(`command: ${command}`);
  const { stdout } = await exec(command, execOptions(projectId));

  if (isBinary(null, stdout)) {
    return "Binary content.";
  } else {
    return stdout;
  }
}

async function branches(projectId) {
  const command = 'git branch -a --format="%(refname)"';
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(projectId));
  log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

async function tags(projectId) {
  const command = "git tag -l";
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(projectId));
  log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

const resolvers = {
  Query: {
    tree: (_, { projectId, revision, prefix }) =>
      tree(projectId, revision, prefix),
    blob: (_, { projectId, revision, path }) => blob(projectId, revision, path),
    branches: (_, { projectId }) => branches(projectId),
    tags: (_, { projectId }) => tags(projectId)
  }
};

const server = new GraphQLServer({ typeDefs, resolvers });
server.start(() => console.log("Server is running on http://localhost:4000"));

// HELPER

// This helper usese `git rev-list <path>` to retrieve the last commit hash
// which touched the given path (directory or file) for the branch given.
async function lastCommitInBranch(path, branch) {
  const command = `git rev-list -n 1 HEAD --branches ${branch} -- "${path}"`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions("FIXME"));
  log(stdout);

  return stdout;
}
