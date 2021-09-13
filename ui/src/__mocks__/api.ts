// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as project from "ui/src/project";
import type * as session from "ui/src/session";
import type * as source from "ui/src/source";

import * as identity from "ui/src/identity";
import * as settings from "ui/src/settings";

type MockedResponse =
  | project.Project
  | project.Project[]
  | session.SessionData
  | source.LocalState
  | null;

export const upstreamProjectMock: project.Project = {
  urn: "rad:git:hwd1yregn1xe4krjs5h7ag5ceut9rwmjssr8e8t4pw6nrwdxgc761o3x4sa",
  metadata: {
    name: "radicle-upstream",
    defaultBranch: "eichhoernchen",
    description:
      "Upstream is a cross-platform desktop client for the radicle code collaboration protocol.",
    maintainers: [],
  },
  stats: {
    branches: 2,
    commits: 22,
    contributors: 222,
  },
};

const surfProjectMock: project.Project = {
  urn: "rad:git:hwd1yref66p4r3z1prxwdjr7ig6ihhrfzsawnc6us4zxtapfukrf6r7mupw",
  metadata: {
    name: "radicle-surf",
    defaultBranch: "schildkroete",
    description: "A code browsing library for VCS file systems",
    maintainers: [],
  },
  stats: {
    branches: 3,
    commits: 33,
    contributors: 333,
  },
};

export const sessionMock: session.SessionData = {
  settings: {
    appearance: {
      theme: settings.Theme.Dark,
      uiFont: settings.UIFont.Inter,
      codeFont: settings.CodeFont.SourceCode,
      hints: {
        showRemoteHelper: true,
      },
    },
    coco: {
      seeds: ["seed.radicle.xyz"],
    },
  },
  identity: identity.fallback,
};

export const localStateMock: source.LocalState = {
  branches: ["main", "other-branch"],
  managed: false,
};

export const get = async (endpoint: string): Promise<MockedResponse> => {
  const [prefix, param] = endpoint.split("/");

  let response: MockedResponse;

  switch (prefix) {
    case "projects":
      response =
        param === "contributed"
          ? [upstreamProjectMock, surfProjectMock]
          : upstreamProjectMock;
      break;
    case "session":
      response = sessionMock;
      break;
    case "source":
      response = param === "local-state" ? localStateMock : null;
      break;
  }

  return new Promise(resolve => resolve(response));
};

// When we want to ensure a function is called with certain parameters, but we don't
// care as much about response data (or if it doesn't have a response), we can use jest.fn()
// to track it
export const post = jest.fn(() => Promise.resolve());
export const del = jest.fn(() => Promise.resolve());
export const set = jest.fn(() => Promise.resolve());
