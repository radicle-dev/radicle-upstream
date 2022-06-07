// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as Path from "node:path";

import { UpstreamPeer, SEED_URL } from "test/support/peerManager";
import { test, expect } from "test/support/playwright/fixtures";
import * as Support from "test/support";

const projectName = "git-platinum";
const projectDescription = "Platinum files for testing radicle-upstream";
const defaultBranch = "main";

test.describe("source browsing with a single peer", () => {
  let urn: string;
  let peer: UpstreamPeer;

  test.beforeEach(async ({ app, page, peerManager }) => {
    peer = await peerManager.startPeer({ name: "peer" });
    ({ urn } = await createProjectFromPlatinumFixture(peer));
    await page.goto(peer.uiUrl());
    await app.goToProjectByName(projectName);
  });

  test("project header", async ({ app }) => {
    await expect(app.projectScreen.header).toContainText(projectName);
    await expect(app.projectScreen.header).toContainText(projectDescription);
    // Project ID.
    await expect(app.projectScreen.header).toContainText(
      `${urn.replace("rad:git:", "").substring(0, 8)}…${urn.slice(-8)}`
    );
  });

  test("project tab navigation", async ({ app }) => {
    // The Files tab is selected by default.
    {
      await expect(app.projectScreen.filesTabButton.locator("p")).toHaveClass(
        /active/
      );
      await expect(
        app.projectScreen.commitsTabButton.locator("p")
      ).not.toHaveClass(/active/);
      await expect(
        app.projectScreen.patchesTabButton.locator("p")
      ).not.toHaveClass(/active/);
    }

    // Clicking the project title navigates back to the Files tab.
    {
      await app.projectScreen.goToCommitsTab();
      await expect(app.projectScreen.commitsTabButton.locator("p")).toHaveClass(
        /active/
      );

      await app.projectScreen.header.locator(`text=${projectName}`).click();
      await expect(app.projectScreen.filesTabButton.locator("p")).toHaveClass(
        /active/
      );
      await expect(
        app.projectScreen.commitsTabButton.locator("p")
      ).not.toHaveClass(/active/);
    }

    // Clicking source tree items does not mess with the tab bar active state.
    {
      await app.projectScreen.goToFilesTab();
      await expect(app.projectScreen.filesTabButton.locator("p")).toHaveClass(
        /active/
      );

      await app.projectScreen.sourceTree.locator("text=bin").click();
      await app.projectScreen.sourceTree.locator("text=cat").click();

      await expect(app.projectScreen.filesTabButton.locator("p")).toHaveClass(
        /active/
      );
    }
  });

  test("Files tab", async ({ app }) => {
    // Initial state.
    {
      // The default branch is initially selected and has a "default" badge.
      await expect(app.projectScreen.selectBranchButton).toContainText(
        `${defaultBranch} default`
      );

      // Latest commit teaser is visible.
      await expect(app.projectScreen.commitTeaser).toContainText("a0dd912");
      await expect(app.projectScreen.commitTeaser).toContainText(
        "Add files with special characters in their filenames (#5)"
      );
      await expect(app.projectScreen.commitTeaser).toContainText(
        "Rūdolfs Ošiņš"
      );

      // The README file is shown by default.
      await expect(app.projectScreen.fileView).toContainText("README.md");

      await expect(app.projectScreen.fileView).toContainText(
        "This repository is a data source for the Upstream front-end tests and the radicle-surf unit tests."
      );
      await expect(app.projectScreen.sourceTree).not.toContainText(
        "here-we-are-on-a-dev-branch.lol"
      );
    }

    // Switch to "dev" branch.
    {
      await app.projectScreen.selectBranch("dev");
      await expect(app.projectScreen.selectBranchButton).toContainText("dev");

      await expect(app.projectScreen.commitTeaser).toContainText("27acd68");
      await expect(app.projectScreen.commitTeaser).toContainText(
        "Commit on the dev branch"
      );
      await expect(app.projectScreen.commitTeaser).toContainText(
        "Rūdolfs Ošiņš"
      );

      await expect(app.projectScreen.fileView).toContainText(
        "This repository is a data source for the Upstream front-end tests."
      );

      await expect(app.projectScreen.sourceTree).toContainText(
        "here-we-are-on-a-dev-branch.lol"
      );
      await expect(app.projectScreen.sourceTree).not.toContainText("special");
    }
  });

  test("Commits tab", async ({ app }) => {
    // Initial state.
    {
      await app.projectScreen.goToCommitsTab();
      await expect(app.projectScreen.selectBranchButton).toContainText(
        `${defaultBranch} default`
      );
      await expect(app.projectScreen.commitsTabButton).toContainText("15");
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"]`)
      ).toHaveCount(15);
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"] >> nth=14`)
      ).toContainText("Initial commit FTW!");
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"] >> nth=0`)
      ).toContainText(
        "Add files with special characters in their filenames (#5)"
      );
    }

    // Switch to "dev" branch.
    {
      await app.projectScreen.selectBranch("dev");
      await expect(app.projectScreen.commitsTabButton).toContainText("8");
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"]`)
      ).toHaveCount(8);
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"] >> nth=7`)
      ).toContainText("Initial commit FTW!");
      await expect(
        app.projectScreen.commitList.locator(`[data-cy="commit"] >> nth=0`)
      ).toContainText("Commit on the dev branch");
    }
  });

  test("source tree", async ({ app }) => {
    test.setTimeout(60_000);

    // Shows the top-level source tree.
    {
      // Directories
      await expect(app.projectScreen.sourceTree).toContainText("bin");
      await expect(app.projectScreen.sourceTree).toContainText("special");
      await expect(app.projectScreen.sourceTree).toContainText("src");
      await expect(app.projectScreen.sourceTree).toContainText("text");
      await expect(app.projectScreen.sourceTree).toContainText("this");

      // Hidden files.
      await expect(app.projectScreen.sourceTree).toContainText(
        ".i-am-well-hidden"
      );
      await expect(app.projectScreen.sourceTree).toContainText(
        ".i-too-am-hidden"
      );

      // Regular files.
      await expect(app.projectScreen.sourceTree).toContainText("README.md");
    }

    // Highlights the selected file in the source tree.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=bin").click();
      await app.projectScreen.sourceTree.locator("text=cat").click();
      await expect(
        app.projectScreen.sourceTree.locator(".file", { hasText: "cat" })
      ).toHaveClass(/active/);
    }

    // Source tree stays expanded while navigating between files or branches.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=bin").click();
      await app.projectScreen.sourceTree.locator("text=src").click();

      await expect(app.projectScreen.sourceTree).toContainText("cat");
      await expect(app.projectScreen.sourceTree).toContainText("Eval.hs");

      await app.projectScreen.sourceTree
        .locator("text=.i-too-am-hidden")
        .click();

      await expect(app.projectScreen.sourceTree).toContainText("cat");
      await expect(app.projectScreen.sourceTree).toContainText("Eval.hs");

      await app.projectScreen.selectBranch("dev");

      await expect(app.projectScreen.sourceTree).toContainText("cat");
      await expect(app.projectScreen.sourceTree).toContainText("Eval.hs");
    }

    // Source tree gets collapsed when navigating away from the Files tab.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=bin").click();
      await app.projectScreen.sourceTree.locator("text=src").click();

      await expect(app.projectScreen.sourceTree).toContainText("cat");
      await expect(app.projectScreen.sourceTree).toContainText("Eval.hs");

      await app.projectScreen.goToCommitsTab();
      await app.projectScreen.goToFilesTab();

      await expect(app.projectScreen.sourceTree).not.toContainText("cat");
      await expect(app.projectScreen.sourceTree).not.toContainText("Eval.hs");
    }

    // Can navigate deeply nested directories.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=this").click();
      await app.projectScreen.sourceTree.locator("text=/^is$/").click();
      await app.projectScreen.sourceTree.locator("text=/^a$/").click();
      await app.projectScreen.sourceTree.locator("text=really").click();
      await app.projectScreen.sourceTree.locator("text=deeply").click();
      await app.projectScreen.sourceTree.locator("text=nested").click();
      await app.projectScreen.sourceTree.locator("text=directory").click();
      await app.projectScreen.sourceTree.locator("text=tree").click();
      await app.projectScreen.sourceTree.locator("text=.gitkeep").click();
      await expect(app.projectScreen.fileView).toContainText(
        "this / is / a / really / deeply / nested / directory / tree / .gitkeep"
      );
    }
  });

  test("file contents", async ({ app }) => {
    // Shows file contents.
    {
      await app.projectScreen.sourceTree.locator("text=src").click();
      await app.projectScreen.sourceTree.locator("text=Eval.hs").click();
      await expect(app.projectScreen.fileView).toContainText(
        "module Radicle.Lang.Eval"
      );

      // Shows line numbers.
      await expect(
        app.projectScreen.fileView.locator(".line-numbers")
      ).toContainText("1\n2\n3\n4\n5\n");
    }

    // Code syntax is highlighted.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=src").click();
      await app.projectScreen.sourceTree.locator("text=memory.rs").click();
      await expect(
        app.projectScreen.fileView.locator(
          "text='//! Provides [MemoryClient] to run the registry ledger in memory.'"
        )
      ).toHaveCSS("color", "rgb(101, 115, 126)");
      await expect(
        app.projectScreen.fileView.locator("text='genesis_hash'")
      ).toHaveCSS("color", "rgb(191, 97, 106)");
    }

    // Shows a placeholder for binary files.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=bin").click();
      await app.projectScreen.sourceTree.locator("text=cat").click();
      await expect(app.projectScreen.fileView).toContainText("bin / cat");
      await expect(app.projectScreen.fileView).toContainText("Binary content");
    }

    // Clicking the Files tab navigates to the project-root, i.e the README.
    {
      await app.goToProjectByName(projectName);

      await app.projectScreen.sourceTree.locator("text=text").click();
      await app.projectScreen.sourceTree.locator("text=arrows.txt").click();
      await expect(app.projectScreen.fileView).toContainText(
        "text / arrows.txt"
      );

      await app.projectScreen.goToFilesTab();
      await expect(app.projectScreen.fileView).toContainText("README.md");
    }
  });

  test("revision selector works after a page reload", async ({ app, page }) => {
    await app.projectScreen.selectBranch("dev");
    await expect(app.projectScreen.selectBranchButton).toContainText("dev");

    await page.reload();

    // The source tree reverts back to the default branch.
    await expect(app.projectScreen.selectBranchButton).toContainText(
      defaultBranch
    );
    // Navigation still works.
    await app.projectScreen.sourceTree.locator("text=special").click();
    await expect(app.projectScreen.sourceTree).toContainText("-dash-");
  });

  test("switching between projects", async ({ app }) => {
    await app.projectScreen.selectBranch("dev");
    await expect(app.projectScreen.selectBranchButton).toContainText("dev");

    const anotherProjectName = "another-project";
    const { defaultBranch } = await Support.createProject(
      peer,
      anotherProjectName
    );
    await app.goToProjectByName(anotherProjectName);
    await expect(app.projectScreen.selectBranchButton).toContainText(
      defaultBranch
    );
  });

  test("filenames with special characters", async ({ app }) => {
    await app.projectScreen.sourceTree.locator("text=special").click();

    await app.projectScreen.sourceTree.locator("text=-dash-").click();
    await expect(app.projectScreen.fileView).toContainText("special / -dash-");

    await app.projectScreen.sourceTree.locator("text=...").click();
    await expect(app.projectScreen.fileView).toContainText("special / ...");

    await app.projectScreen.sourceTree.locator("text=:colon:").click();
    await expect(app.projectScreen.fileView).toContainText("special / :colon:");

    await app.projectScreen.sourceTree.locator("text=;semicolon;").click();
    await expect(app.projectScreen.fileView).toContainText(
      "special / ;semicolon;"
    );

    await app.projectScreen.sourceTree.locator("text=@at@").click();
    await expect(app.projectScreen.fileView).toContainText("special / @at@");

    await app.projectScreen.sourceTree.locator("text=_underscore_").click();
    await expect(app.projectScreen.fileView).toContainText(
      "special / _underscore_"
    );

    await app.projectScreen.sourceTree.locator("text=c++").click();
    await expect(app.projectScreen.fileView).toContainText("special / c++");

    await app.projectScreen.sourceTree.locator("text=faux\\path").click();
    await expect(app.projectScreen.fileView).toContainText(
      "special / faux\\path"
    );

    await app.projectScreen.sourceTree
      .locator("text='i need some space'")
      .click();
    await expect(app.projectScreen.fileView).toContainText(
      "special / i need some space"
    );

    await app.projectScreen.sourceTree
      .locator("text=qs?param1=value?param2=value2#hash")
      .click();
    await expect(app.projectScreen.fileView).toContainText(
      "special / qs?param1=value?param2=value2#hash"
    );

    await app.projectScreen.sourceTree.locator("text=~tilde~").click();
    await expect(app.projectScreen.fileView).toContainText("special / ~tilde~");

    await app.projectScreen.sourceTree.locator("text=👹👹👹").click();
    await expect(app.projectScreen.fileView).toContainText("special / 👹👹👹");
  });
});

test("view project from another peer's perspective", async ({
  app,
  page,
  peerManager,
}) => {
  const maintainer = await peerManager.startPeer({ name: "maintainer" });
  const contributor = await peerManager.startPeer({ name: "contributor" });

  const { urn, checkoutPath } = await createProjectFromPlatinumFixture(
    maintainer
  );
  await Support.publishProject(maintainer, urn, checkoutPath);

  const branchName = "contributor-branch";
  const commitMessage = "Contributor's commit on own branch";
  const fileName = "file-on-contributor-branch.txt";

  // Contributor creates and publishes a branch with a commit adding a new file.
  {
    await page.goto(contributor.uiUrl());
    await app.trackProject(urn);
    await app.goToProjectByName(projectName);

    // The maintainer is pre-selected, and has a "maintainer" badge.
    await expect(app.projectScreen.selectPeerButton).toContainText(
      `${maintainer.userHandle} delegate`
    );

    const projectWorkingCopyPath = await Support.forkProject(
      contributor,
      urn,
      projectName
    );

    // After the project is forked, the contributor peer is selected and has
    // a "you" badge.
    await expect(app.projectScreen.selectPeerButton).toContainText(
      `${contributor.userHandle} you`
    );
    await expect(app.projectScreen.commitsTabButton).toContainText("15");

    await contributor.spawn("git", ["checkout", "-b", branchName], {
      cwd: projectWorkingCopyPath,
    });

    await contributor.spawn("touch", [fileName], {
      cwd: projectWorkingCopyPath,
    });

    await contributor.spawn("git", ["add", fileName], {
      cwd: projectWorkingCopyPath,
    });

    await contributor.spawn("git", ["commit", "--message", commitMessage], {
      cwd: projectWorkingCopyPath,
    });

    await Support.publishProject(contributor, urn, projectWorkingCopyPath);

    // FIXME: this test is flaky here.
    await app.goToProjectByName(projectName);

    await app.projectScreen.selectBranch(branchName);

    await expect(app.projectScreen.commitTeaser).toContainText(commitMessage);
    await expect(app.projectScreen.sourceTree).toContainText(fileName);
    await expect(app.projectScreen.commitsTabButton).toContainText("16");
  }

  // Maintainer views the project source from the contributor's perspective.
  {
    await page.goto(maintainer.uiUrl());
    await app.goToProjectByName(projectName);
    await app.projectScreen.addRemotes([contributor.peerId]);
    await app.projectScreen.selectPeer(contributor.userHandle);

    // The contributor's peer does not have a badge.
    await expect(app.projectScreen.selectPeerButton).toContainText(
      `${contributor.userHandle}`
    );
    await expect(app.projectScreen.selectPeerButton).not.toContainText(
      "delegate"
    );
    await expect(app.projectScreen.selectPeerButton).not.toContainText("you");

    // The default branch is initially selected.
    await expect(app.projectScreen.selectBranchButton).toContainText(
      `${defaultBranch} default`
    );
    await expect(app.projectScreen.commitsTabButton).toContainText("15");
    await expect(app.projectScreen.commitTeaser).toContainText(
      "Add files with special characters in their filenames (#5)"
    );
    await expect(app.projectScreen.sourceTree).not.toContainText(fileName);

    // Switch to the branch that the contributor created.
    await app.projectScreen.selectBranch(branchName);
    await expect(app.projectScreen.commitsTabButton).toContainText("16");
    await expect(app.projectScreen.commitTeaser).toContainText(commitMessage);
    await expect(app.projectScreen.sourceTree).toContainText(fileName);
  }
});

// Create a project from the platinum fixture using the rad CLI.
export async function createProjectFromPlatinumFixture(
  peer: UpstreamPeer
): Promise<{
  urn: string;
  checkoutPath: string;
}> {
  const checkoutPath = Path.join(peer.checkoutPath, projectName);

  await peer.spawn("git", [
    "clone",
    Path.join(__dirname, "..", "fixtures", projectName),
    checkoutPath,
  ]);

  await peer.spawn("git", ["checkout", "dev"], {
    cwd: checkoutPath,
  });

  await peer.spawn("git", ["checkout", defaultBranch], {
    cwd: checkoutPath,
  });

  await peer.spawn(
    "rad",
    [
      "init",
      "--name",
      projectName,
      "--default-branch",
      defaultBranch,
      "--description",
      projectDescription,
    ],
    {
      cwd: checkoutPath,
    }
  );

  const { stdout: urn } = await peer.spawn("rad", ["inspect"], {
    cwd: checkoutPath,
  });

  await peer.spawn("git", ["config", "--add", "rad.seed", SEED_URL], {
    cwd: checkoutPath,
  });

  return {
    urn,
    checkoutPath,
  };
}
