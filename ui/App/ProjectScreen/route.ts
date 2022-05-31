// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as project from "ui/src/project";

import * as lodash from "lodash";

import * as ensResolver from "ui/src/org/ensResolver";
import * as theGraphApi from "ui/src/org/theGraphApi";
import * as wallet from "ui/src/wallet";

export interface Params {
  urn: string;
  activeView: ProjectView;
}

export type PatchView = "commits" | "discussion";

export interface LoadedRoute {
  type: "project";
  urn: string;
  activeView: ProjectView;
  anchors: project.ConfirmedAnchor[];
}

export type ProjectView =
  | { type: "files" }
  | { type: "commits" }
  | { type: "commit"; commitHash: string }
  | { type: "patches"; filter: "open" | "closed" | "all" }
  | {
      type: "patch";
      id: string;
      peerId: string;
      view: PatchView;
    }
  | { type: "anchors" };

export async function load(params: Params): Promise<LoadedRoute> {
  let anchors: project.ConfirmedAnchor[] = [];

  if (wallet.isConnected()) {
    anchors = lodash.sortBy(
      await theGraphApi.getProjectAnchors(params.urn),
      "timestamp"
    );

    if (
      params.activeView.type === "anchors" ||
      params.activeView.type === "commit"
    ) {
      await Promise.all(
        anchors.map(async anchor => {
          const registration = await ensResolver.getCachedRegistrationByAddress(
            anchor.orgAddress
          );
          anchor.registration = registration;
          return anchor;
        })
      );
    }
  }

  return {
    type: "project",
    urn: params.urn,
    activeView: params.activeView,
    anchors,
  };
}
