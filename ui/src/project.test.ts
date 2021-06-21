// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { get } from "svelte/store";

import * as project from "./project";
import * as remote from "./remote";
import { UPSTREAM_DEFAULT_BRANCH } from "./config";

import { localStateMock } from "./__mocks__/api";

jest.mock("./api");

describe("creating a project", () => {
  describe("path validation", () => {
    it("formats project names correctly", () => {
      const acceptable = "new_project";
      const withSpaces = "new project";

      // formats spaces
      expect(project.formatNameInput(withSpaces)).toEqual("new-project");

      // doesn't mess with names that are already ok
      expect(project.formatNameInput(acceptable)).toEqual(acceptable);
    });

    it("extract project names correctly from a repository path", () => {
      expect(
        project.extractName("screaming/somewhere/in/the/machine/my-project")
      ).toEqual("my-project");
    });

    it("fetches local state and sets store accordingly", () => {
      const validation = project.repositoryPathValidationStore(false);
      validation.validate("/repository/path");

      expect(get(project.localState)).toEqual({
        status: remote.Status.Loading,
      });

      process.nextTick(() => {
        expect(get(project.localState)).toEqual({
          status: remote.Status.Success,
          data: localStateMock,
        });
      });
    });

    it("re-sets the local state error on validation start", () => {
      project.localStateError.set("test-error");
      const validation = project.repositoryPathValidationStore(false);
      validation.validate("/repository/path");

      expect(get(project.localStateError)).toEqual("");
    });

    it("sets the default branch", () => {
      project.defaultBranch.set("");
      const validation = project.repositoryPathValidationStore(false);
      validation.validate("/repository/path");

      process.nextTick(() => {
        expect(get(project.defaultBranch)).toEqual(UPSTREAM_DEFAULT_BRANCH);
      });
    });
  });
});
