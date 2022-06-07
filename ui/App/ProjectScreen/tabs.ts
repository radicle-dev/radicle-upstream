// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as projectRoute from "./route";

import { selectPath } from "ui/src/screen/project/source";
import * as router from "ui/src/router";

import CommitIcon from "design-system/icons/Commit.svelte";
import FileIcon from "design-system/icons/File.svelte";
import RevisionIcon from "design-system/icons/Revision.svelte";

import { Tab } from "ui/App/ScreenLayout/TabBar";

export function makeTabs({
  projectUrn,
  activeViewType,
  commitCount,
  patchCount,
}: {
  projectUrn: string;
  activeViewType: projectRoute.ProjectView["type"];
  commitCount: number;
  patchCount: number;
}): Tab[] {
  return [
    {
      title: "Files",
      active: activeViewType === "files",
      icon: FileIcon,
      onClick: () => {
        if (activeViewType === "files") {
          selectPath("");
        } else {
          router.push({
            type: "project",
            params: {
              urn: projectUrn,
              activeView: { type: "files" },
            },
          });
        }
      },
    },
    {
      title: "Commits",
      active: activeViewType === "commits",
      icon: CommitIcon,
      counter: commitCount,
      onClick: () => {
        router.push({
          type: "project",
          params: {
            urn: projectUrn,
            activeView: { type: "commits" },
          },
        });
      },
    },
    {
      title: "Patches",
      active: activeViewType === "patches",
      icon: RevisionIcon,
      counter: patchCount,
      onClick: () => {
        router.push({
          type: "project",
          params: {
            urn: projectUrn,
            activeView: { type: "patches", filter: "open" },
          },
        });
      },
    },
  ];
}
