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
    it("sets the local state", () => {
      project.localState.set("test");
      const validation = project.repositoryPathValidationStore(false);
      validation.validate("/repository/path");

      // resetting the local state on validation start
      expect(get(project.localState)).toEqual("");

      process.nextTick(() => {
        expect(get(project.localState)).toEqual(localStateMock);
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

describe("registering a project", () => {
  it("sends a correctly-formatted POST request to the api", () => {
    project
      .register(
        project.Domain.Org,
        "radicle",
        "radicle-link",
        22,
        "this_is_a_coco_id"
      )
      .catch(reason => {
        console.error("Project registration failed: ", reason);
      });

    expect(api.post).toHaveBeenLastCalledWith(
      "orgs/radicle/projects/radicle-link",
      {
        maybeCocoId: "this_is_a_coco_id",
        transactionFee: 22,
      }
    );
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

describe("fetching a list of projects for user profile", () => {
  it("creates and updates a store", () => {
    const store = project.projects;

    expect(get(store)).toEqual({ status: remote.Status.Loading });

    // Store doesn't fetch until it has a subscriber
    store.subscribe(() => null);

    process.nextTick(() => {
      expect(get(store)).toEqual({
        status: remote.Status.Success,
        data: [upstreamProjectMock, surfProjectMock],
      });
    });
  });
});
