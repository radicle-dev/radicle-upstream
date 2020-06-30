import { Org } from "../org";
import { User } from "../user";
import { Project } from "../project";

type MockedResponse = Org | Project | Project[] | User | null;

// just to give an idea of how we'd stub the api with other endpoints
const userMock: User = {
  handle: "rafalca",
};

export const orgMock: Org = {
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

export const upstreamProjectMock: Project = {
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

export const surfProjectMock: Project = {
  id: "%rad:git:hwd1yref66p4r3z1prxwdjr7ig6ihhrfzsawnc6us4zxtapfukrf6r7mupw",
  shareableEntityIdentifier: "sos@{}",
  metadata: {
    name: "radicle-surf",
    defaultBranch: "schildkroete",
    description: "A code browsing library for VCS file systems",
  },
  registration: undefined,
  stats: {
    branches: 3,
    commits: 33,
    contributors: 333,
  },
};

export const get = async (endpoint: string): Promise<MockedResponse> => {
  const [prefix, param] = endpoint.split("/");

  let response: MockedResponse;

  switch (prefix) {
    case "orgs":
      response = param === "radicle" ? orgMock : null;
      break;
    //
    case "user":
      response = userMock;
      break;
    case "projects":
      response = param
        ? upstreamProjectMock
        : [upstreamProjectMock, surfProjectMock];
      break;
  }

  return new Promise(resolve => resolve(response));
};

// When we want to ensure a function is called with certain parameters, but we don't
// care as much about response data (or if it doesn't have a response), we can use jest.fn()
// to track it
export const post = jest.fn();
