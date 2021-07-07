// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as org from "ui/src/org";
import { unreachable } from "ui/src/unreachable";

export interface Params {
  address: string;
  view: View;
}

export type View = "projects" | "members";

export type LoadedRoute = SingleSigLoaded | MultiSigLoaded;

export type MultiSigView =
  | {
      type: "projects";
      anchors: org.OrgAnchors;
      gnosisSafeAddress: string;
      projectCount: number;
    }
  | {
      type: "members";
      threshold: number;
      members: org.Member[];
    };

interface MultiSigLoaded {
  type: "multiSigOrg";
  address: string;
  gnosisSafeAddress: string;
  view: MultiSigView;
  threshold: number;
  members: org.Member[];
}

interface SingleSigLoaded {
  type: "singleSigOrg";
  address: string;
  owner: string;
  projectCount: number;
  anchors: org.OrgAnchors;
}

export async function load(params: Params): Promise<LoadedRoute> {
  const owner = await org.getOwner(params.address);
  const projectCount = await org.getProjectCount();
  const anchors = await org.resolveProjectAnchors(params.address, owner);

  switch (owner.type) {
    case "gnosis-safe": {
      if (params.view === "projects") {
        return {
          type: "multiSigOrg",
          address: params.address,
          gnosisSafeAddress: owner.address,
          members: owner.members,
          threshold: owner.threshold,
          view: {
            type: "projects",
            anchors,
            gnosisSafeAddress: owner.address,
            projectCount,
          },
        };
      } else if (params.view === "members") {
        return {
          type: "multiSigOrg",
          address: params.address,
          gnosisSafeAddress: owner.address,
          members: owner.members,
          threshold: owner.threshold,
          view: {
            type: "members",
            members: owner.members,
            threshold: owner.threshold,
          },
        };
      } else {
        return unreachable(params.view);
      }
    }
    case "wallet": {
      return {
        type: "singleSigOrg",
        address: params.address,
        owner: owner.address,
        projectCount,
        anchors,
      };
    }
  }
}
