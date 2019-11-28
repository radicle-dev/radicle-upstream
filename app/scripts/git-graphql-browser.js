import { GraphQLServer } from "graphql-yoga";
import { isBinary } from "istextorbinary";
import util from "util";
import path from "path";

const exec = util.promisify(require("child_process").exec);

function anonymize(str) {
  return str
    .split("")
    .reduce(
      (prevHash, currVal) =>
        ((prevHash << 5) - prevHash + currVal.charCodeAt(0)) | 0,
      0
    );
}

const typeDefs = `
  scalar Datetime

  type Person {
    name: String
    email: String
    avatar: String
  }

  type Commit {
    sha1: String!
    author: Person!
    authorDate: Datetime!
    subject: String!
    body: String!
    message: String!
  }

  type Info {
    name: String!
    size: Int!
    isDirectory: Boolean!
    mode: String!
    lastCommit: Commit!
  }

  type Blob {
    content: String!
    info: Info!
  }

  type Tree {
    path: String
    info: Info
    entries: [TreeEntry!]!
  }

  type TreeEntry {
    path: String
    info: Info
  }

  type Query {
    blob(projectId: String!, revision: String!, path: String!): Blob!
    tree(projectId: String!, revision: String!, prefix: String!): Tree!

    commit(projectId: String!, sha1: String!): Commit

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

async function commit(projectId, sha1) {
  log(`projectId: ${projectId}`);
  log(`sha1: ${sha1}`);

  const delimiter = "<<<<<<<<<<";

  const command =
    `git show --quiet --pretty=format:'` +
    `%an${delimiter}` +
    `%ae${delimiter}` +
    `%aD${delimiter}` +
    `%s${delimiter}` +
    `%b' ${sha1}`;

  log(`command: ${command}`);
  const { stdout } = await exec(command, execOptions(projectId));

  const [authorName, authorEmail, authorDate, subject, body] = stdout.split(
    "<<<<<<<<<<"
  );

  return {
    sha1: sha1,
    author: {
      name: authorName,
      email: authorEmail,
      avatar: `https://avatars.dicebear.com/v2/human/${anonymize(
        authorName
      )}.svg`
    },
    authorDate: authorDate,
    subject: subject,
    body: body,
    message: subject + "\n\n" + body
  };
}

async function tree(projectId, revision, prefix) {
  log(`projectId: ${projectId}`);
  log(`revision: ${revision}`);
  log(`prefix: ${prefix}`);

  // strip any leading /
  prefix = prefix.replace(/^\//, "");
  // add trailing / if none is present
  if (!prefix.match(/\/$/)) {
    prefix = prefix + "/";
  }

  const command = `git ls-tree --long ${revision} ./${prefix}`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(projectId));

  const listOfPromises = stdout
    .split("\n") // split into rows
    .filter(el => el !== "") // throw out empty rows
    .map(async row => {
      const [mode, treeOrBlob, objectSha, sizeOrDash, nameWithPath] = row.split(
        /\s+/
      );

      const lastCommitInBranchSha1 = await lastCommitInBranch(
        projectId,
        nameWithPath,
        objectSha
      );
      const lastCommit = await commit(projectId, lastCommitInBranchSha1);

      return {
        path: nameWithPath,
        info: {
          mode: mode,
          isDirectory: treeOrBlob === "tree",
          lastCommit: lastCommit,
          size: sizeOrDash === "-" ? -1 : parseInt(sizeOrDash),
          name: nameWithPath.replace(prefix, "")
        }
      };
    });

  const list = await Promise.all(listOfPromises);

  const sortedList = list.sort(function(a, b) {
    // sort directories first, then files alphabetically
    if (a.info.isDirectory && !b.info.isDirectory) return -1;
    if (!a.info.isDirectory && b.info.isDirectory) return 1;
    if (a.info.toLowerCase > b.info.toLowerCase) return 1;
  });

  const lastCommitInBranchSha1 = await lastCommitInBranch(
    projectId,
    `./${prefix}`,
    revision
  );
  const lastCommit = await commit(projectId, lastCommitInBranchSha1);

  return {
    path: prefix,
    info: {
      lastCommit: lastCommit,
      name: prefix.split("/").slice(-1)[0],
      size: -1,
      mode: "TODO",
      isDirectory: true
    },
    entries: sortedList
  };
}

async function blob(projectId, revision, path) {
  log(`projectId: ${projectId}`);
  log(`revision: ${revision}`);
  log(`path: ${path}`);

  const blobCommand = `git show ${revision}:${path}`;
  const { stdout: blob } = await exec(blobCommand, execOptions(projectId));

  const metadataCommand = `git ls-tree --long ${revision} ${path}`;
  const { stdout: metadata } = await exec(
    metadataCommand,
    execOptions(projectId)
  );

  const [mode, objectType, sha1, size, filePath] = metadata.split(/\s+/);

  const lastCommitInBranchSha1 = await lastCommitInBranch(
    projectId,
    path,
    revision
  );
  const lastCommit = await commit(projectId, lastCommitInBranchSha1);

  const blobEntry = {
    content: isBinary(null, blob) ? "ఠ ͟ಠ Binary content." : blob,
    info: {
      mode: mode,
      isDirectory: objectType === "tree",
      lastCommit: lastCommit,
      size: size,
      name: path.split("/").slice(-1)[0]
    }
  };

  return blobEntry;
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
    tags: (_, { projectId }) => tags(projectId),
    commit: (_, { projectId, sha1 }) => commit(projectId, sha1)
  }
};

const server = new GraphQLServer({ typeDefs, resolvers });
server.start(() => console.log("Server is running on http://localhost:4000"));

// HELPER

// This helper usese `git rev-list <path>` to retrieve the last commit hash
// which touched the given path (directory or file) for the branch given.
async function lastCommitInBranch(projectId, path, branch) {
  const command = `git rev-list -n 1 HEAD --branches ${branch} -- "${path}"`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(projectId));
  log(stdout);

  return stdout.trim();
}
