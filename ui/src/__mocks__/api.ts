import { Org } from "../org";
import { User } from "../user";
import { Project } from "../project";

type MockedResponse = Org | Project | User | null;

// just to give an idea of how we'd stub the api with other endpoints
const userMock: User = {
  handle: "rafalca",
};

const radicleMock: Org = {
  id: "radicle",
  shareableEntityIdentifier: "radicle@123abcd.git",
  avatarFallback: {
    background: {
      r: 255,
      g: 67,
      b: 34,
    },
    emoji: "🔥",
  },
  members: [userMock],
};

const projectMock: Project = {
  id: "%rad:git:hwd1yregn1xe4krjs5h7ag5ceut9rwmjssr8e8t4pw6nrwdxgc761o3x4sa",
  shareableEntityIdentifier: "sos@{}",
  metadata: {
    name: "radicle-upstream",
    defaultBranch: "eichhoernchen",
    description:
      "Upstream is a cross-platform desktop client for the radicle code collaboration and registry protocols.",
  },
  registration: undefined,
  stats: {
    branches: 2,
    commits: 22,
    contributors: 222,
  },
};

export const get = async (endpoint: string): Promise<MockedResponse> => {
  const [prefix, param] = endpoint.split("/");

  let response: MockedResponse;

  switch (prefix) {
    case "orgs":
      response = param === "radicle" ? radicleMock : null;
      break;
    case "user":
      response = userMock;
      break;
    case "project":
      response = projectMock;
      break;
  }

  return new Promise(resolve => resolve(response));
};
