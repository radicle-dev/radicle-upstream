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
    ls(projectId: String!, revision: String!, prefix: String!): [Entry]!
    cat(projectId: String!, revision: String!, path: String!): String!
    branches(projectId: String!): [String]!
    tags(projectId: String!): [String]!
  }
`;

const debug = false;

async function ls(projectId, revision, prefix) {
  debug && console.log(`projectId: ${projectId}`);
  debug && console.log(`revision: ${revision}`);
  debug && console.log(`prefix: ${prefix}`);

  const repoBasePath = path.resolve(__dirname, "../");
  debug && console.log(`repoBasePath: ${repoBasePath}`);

  const command = `git ls-tree --long ${revision} ${repoBasePath}${prefix}`;
  debug && console.log(`command: ${command}`);
  const { stdout } = await exec(command);

  const relativePrefix = prefix.replace(/^\//, "");

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

      debug && console.log(`nameWithPath: ${nameWithPath}`);
      const name = nameWithPath.replace(new RegExp(`^${relativePrefix}`), "");
      debug && console.log(`name: ${name}`);
      debug && console.log("\n");

      return {
        path: prefix + name,
        info: {
          mode: mode,
          isDirectory: treeOrBlob === "tree",
          lastCommit: lastCommit,
          size: sizeOrDash === "-" ? 0 : parseInt(sizeOrDash),
          name: name
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

async function cat(projectId, revision, path) {
  debug && console.log(`projectId: ${projectId}`);
  debug && console.log(`revision: ${revision}`);
  debug && console.log(`path: ${path}`);

  const command = `git show ${revision}:${path}`;
  debug && console.log(`command: ${command}`);
  const { stdout } = await exec(command);

  if (isBinary(null, stdout)) {
    return "Binary content.";
  } else {
    return stdout;
  }
}

async function branches(_projectId) {
  const command = 'git branch -a --format="%(refname)"';
  debug && console.log(`command: ${command}`);

  const { stdout } = await exec(command);
  debug && console.log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

async function tags(_projectId) {
  const command = "git tag -l";
  debug && console.log(`command: ${command}`);

  const { stdout } = await exec(command);
  debug && console.log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

const resolvers = {
  Query: {
    ls: (_, { projectId, revision, prefix }) => ls(projectId, revision, prefix),
    cat: (_, { projectId, revision, path }) => cat(projectId, revision, path),
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
  debug && console.log(`command: ${command}`);

  const { stdout } = await exec(command);
  debug && console.log(stdout);

  return stdout;
}
