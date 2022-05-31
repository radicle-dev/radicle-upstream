// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Route } from "./definition";
import { routeToUri, uriToRoute } from "./path";

describe("uriToRoute", () => {
  it("returns undefined if it can't parse the path", () => {
    expect(uriToRoute("/not/a/recognised/path")).toBeUndefined();
  });

  it("parses patch paths", () => {
    const route: Route = {
      type: "project",
      params: {
        urn: "rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o",
        activeView: {
          type: "patch",
          peerId: "hyn5r6yejjco8r77yf7gu6gqsetgjsqt5oitpzu5eu791wej6p3xz6",
          id: "no-session-kv/patch",
          view: "commits",
        },
      },
    };
    expect(uriToRoute(routeToUri(route))).toEqual(route);
  });
});
