import { get } from "svelte/store";

import * as api from "./api";
import * as project from "./project";
import * as remote from "./remote";
import { DEFAULT_BRANCH_FOR_NEW_PROJECTS } from "./config";

import {
  localStateMock,
  surfProjectMock,
  upstreamProjectMock,
} from "./__mocks__/api";

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

      // resetting the default branch on validation start
      expect(get(project.defaultBranch)).toEqual(
        DEFAULT_BRANCH_FOR_NEW_PROJECTS
      );

      process.nextTick(() => {
        expect(get(project.defaultBranch)).toEqual("main");
      });
    });
  });

  it("sends a correctly-formatted POST request to api", () => {
    project
      .create({
        defaultBranch: "trunk",
        description: "surfing",
        repo: {
          type: project.RepoType.New,
          name: "radicle-surf",
          path: "somewhere/in/the/machine",
        },
      })
      .catch(reason => {
        console.error("Project creation failed: ", reason);
      });

    expect(api.post).toHaveBeenCalledWith("projects", {
      defaultBranch: "trunk",
      description: "surfing",
      repo: {
        type: project.RepoType.New,
        name: "radicle-surf",
        path: "somewhere/in/the/machine",
      },
    });
  });
});

describe("fetching a project", () => {
  it("creates and updates a store", () => {
    const store = project.project;
    project.fetch({ id: "radicle" });

    expect(get(store)).toEqual({ status: remote.Status.Loading });

    process.nextTick(() => {
      expect(get(store)).toEqual({
        status: remote.Status.Success,
        data: upstreamProjectMock,
      });
    });
  });
});
