// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as project from "./project";

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
  });
});
