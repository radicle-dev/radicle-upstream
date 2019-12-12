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

  input IdInput {
    domain: String!
    name: String!
  }

  enum ObjectType {
    TREE
    BLOB
  }

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
    objectType: ObjectType!
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
    blob(projectId: IdInput!, revision: String!, path: String!): Blob!
    tree(projectId: IdInput!, revision: String!, prefix: String!): Tree!

    commit(projectId: IdInput!, sha1: String!): Commit

    branches(projectId: IdInput!): [String]!
    tags(projectId: IdInput!): [String]!
  }
`;

const debug = false;

// TODO: implement actual lookup once we have mock data with stable IDs
const projectRepoPathById = _projectId => {
  switch (process.argv[2]) {
    case "--test":
      return path.resolve(__dirname, "../../fixtures/git-platinum");
    // in dev mode serve the upstream app repo itself
    default:
      return path.resolve(__dirname, "../../");
  }
};

const execOptions = (domain, name) => ({
  maxBuffer: 1024 * 1000,
  cwd: projectRepoPathById(domain, name)
});

const log = message => {
  debug && console.log(message);
};

async function commit(domain, name, sha1) {
  log(`commit() domain: ${domain}, name: ${name}`);
  log(`commit() sha1: ${sha1}`);

  const delimiter = "<<<<<<<<<<";

  const command =
    `git show --quiet --pretty=format:'` +
    `%an${delimiter}` +
    `%ae${delimiter}` +
    `%aD${delimiter}` +
    `%s${delimiter}` +
    `%b' ${sha1}`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(domain, name));

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

async function tree(domain, name, revision, prefix) {
  log(`tree() domain: ${domain}, name: ${name}`);
  log(`tree() revision: ${revision}`);
  log(`tree() prefix: ${prefix}`);

  // strip any leading /
  prefix = prefix.replace(/^\//, "");
  // add trailing / if none is present
  if (!prefix.match(/\/$/)) {
    prefix = prefix + "/";
  }

  const command = `git ls-tree --long ${revision} ./${prefix}`;
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(domain, name));

  const listOfPromises = stdout
    .split("\n") // split into rows
    .filter(el => el !== "") // throw out empty rows
    .map(async row => {
      const [mode, objectType, objectSha, sizeOrDash, nameWithPath] = row.split(
        /\s+/
      );

      const lastCommitInBranchSha1 = await lastCommitInBranch(
        domain,
        name,
        nameWithPath,
        objectSha
      );
      const lastCommit = await commit(domain, name, lastCommitInBranchSha1);

      return {
        path: nameWithPath,
        info: {
          mode: mode,
          objectType: objectType.toUpperCase(),
          lastCommit: lastCommit,
          size: sizeOrDash === "-" ? -1 : parseInt(sizeOrDash),
          name: nameWithPath.replace(prefix, "")
        }
      };
    });

  const list = await Promise.all(listOfPromises);

  const sortedList = list.sort(function(a, b) {
    // sort directories first, then files alphabetically
    if (a.info.objectType === "TREE" && b.info.objectType === "BLOB") return -1;
    if (a.info.objectType === "BLOB" && b.info.objectType === "TREE") return 1;
    if (a.info.toLowerCase > b.info.toLowerCase) return 1;
  });

  const lastCommitInBranchSha1 = await lastCommitInBranch(
    domain,
    name,
    `./${prefix}`,
    revision
  );
  const lastCommit = await commit(domain, name, lastCommitInBranchSha1);

  return {
    path: prefix,
    info: {
      lastCommit: lastCommit,
      name: prefix.split("/").slice(-1)[0],
      size: -1,
      mode: "TODO",
      objectType: "TREE"
    },
    entries: sortedList
  };
}

async function blob(domain, name, revision, path) {
  log(`blob() domain: ${domain}, name: ${name}`);
  log(`blob() revision: ${revision}`);
  log(`blob() path: ${path}`);

  const blobCommand = `git show ${revision}:${path}`;
  const { stdout: blob } = await exec(blobCommand, execOptions(domain, name));

  const metadataCommand = `git ls-tree --long ${revision} ${path}`;
  const { stdout: metadata } = await exec(
    metadataCommand,
    execOptions(domain, name)
  );

  const [mode, objectType, sha1, size, filePath] = metadata.split(/\s+/);

  const lastCommitInBranchSha1 = await lastCommitInBranch(
    domain,
    name,
    path,
    revision
  );
  const lastCommit = await commit(domain, name, lastCommitInBranchSha1);

  const blobEntry = {
    content: isBinary(null, blob) ? "ఠ ͟ಠ Binary content." : blob,
    info: {
      mode: mode,
      objectType: objectType.toUpperCase(),
      lastCommit: lastCommit,
      size: size,
      name: path.split("/").slice(-1)[0]
    }
  };

  return blobEntry;
}

async function branches(domain, name) {
  log(`branches() domain: ${domain}, name: ${name}`);

  const command = 'git branch -a --format="%(refname)"';
  log(`command: ${command}`);

  const { stdout } = await exec(command, execOptions(domain, name));
  log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

async function tags(domain, name) {
  log(`tags() domain: ${domain}, name: ${name}`);

  const command = "git tag -l";
  log(`tags() command: ${command}`);

  const { stdout } = await exec(command, execOptions(domain, name));
  log(stdout);

  return stdout
    .split("\n") // split into rows
    .filter(el => el !== ""); // throw out empty rows
}

const resolvers = {
  Query: {
    tree: (_, { projectId, revision, prefix }) =>
      tree(projectId.domain, projectId.name, revision, prefix),
    blob: (_, { projectId, revision, path }) =>
      blob(projectId.domain, projectId.name, revision, path),
    branches: (_, { projectId }) => branches(projectId.domain, projectId.name),
    tags: (_, { projectId }) => tags(projectId.domain, projectId.name),
    commit: (_, { projectId, sha1 }) =>
      commit(projectId.domain, projectId.name, sha1)
  }
};

const server = new GraphQLServer({ typeDefs, resolvers });
server.start(() => console.log("Server is running on http://localhost:4000"));

// HELPER

// This helper usese `git rev-list <path>` to retrieve the last commit hash
// which touched the given path (directory or file) for the branch given.
async function lastCommitInBranch(domain, name, path, branch) {
  log(`lastCommitInBranch() domain: ${domain}, name: ${name}`);

  const command = `git rev-list -n 1 HEAD --branches ${branch} -- "${path}"`;
  log(`lastCommitInBranch() command: ${command}`);

  const { stdout } = await exec(command, execOptions(domain, name));
  log(stdout);

  return stdout.trim();
}
