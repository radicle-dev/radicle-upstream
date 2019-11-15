import { GraphQLServer } from "graphql-yoga";
const util = require("util");
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
    ls(projectId: String!, head: String!, prefix: String!): [Entry]!
  }
`;

const debug = false;

async function ls(projectId, head, prefix) {
  debug && console.log(`projectId: ${projectId}`);
  debug && console.log(`head: ${head}`);
  debug && console.log(`prefix: ${prefix}`);

  const repoBasePath = path.resolve(__dirname, "../");
  debug && console.log(`repoBasePath: ${repoBasePath}`);

  const command = `git ls-tree --long ${head} ${repoBasePath}${prefix}`;
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

const resolvers = {
  Query: {
    ls: (_, { projectId, head, prefix }) => ls(projectId, head, prefix)
  }
};

const server = new GraphQLServer({ typeDefs, resolvers });
server.start(() => console.log("Server is running on http://localhost:4000"));
