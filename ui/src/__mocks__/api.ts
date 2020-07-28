import * as org from "../org";
import * as project from "../project";
import * as session from "../session";
import * as settings from "../settings";
import * as user from "../user";

type MockedResponse =
  | org.Org
  | project.Project
  | project.Project[]
  | session.Session
  | user.User
  | null;

// just to give an idea of how we'd stub the api with other endpoints
const userMock: user.User = {
  handle: "rafalca",
};

export const orgMock: org.Org = {
  id: "radicle",
  accountId: "5EEAHNstTd1QGN3889TZNZ24U3PVVEvDbRp8S7FyUwmN2LtN",
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

export const upstreamProjectMock: project.Project = {
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

export const surfProjectMock: project.Project = {
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

export const sessionMock: session.Session = {
  orgs: [],
  permissions: {
    registerHandle: true,
    registerOrg: false,
    registerProject: false,
  },
  settings: {
    appearance: {
      theme: settings.Theme.Light,
    },
    registry: {
      network: settings.Network.Emulator,
    },
  },
  registrationFee: {
    user: 10,
    org: 10,
    project: undefined,
    member: undefined,
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
    case "session":
      response = sessionMock;
  }

  return new Promise(resolve => resolve(response));
};

// When we want to ensure a function is called with certain parameters, but we don't
// care as much about response data (or if it doesn't have a response), we can use jest.fn()
// to track it
export const post = jest.fn(() => Promise.resolve());
export const del = jest.fn(() => Promise.resolve());
export const set = jest.fn(() => Promise.resolve());
