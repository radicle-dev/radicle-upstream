// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.
import * as router from "ui/src/router";
import * as session from "ui/src/session";

export async function openUserProfile(userUrn: string): Promise<void> {
  if (userUrn === session.unsealed().identity.urn) {
    router.push({ type: "profile", activeTab: "projects" });
  } else {
    router.push({
      type: "userProfile",
      urn: userUrn,
    });
  }
}
