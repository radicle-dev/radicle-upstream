// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type { Route } from "./definition";
import { routeToPath, pathToRoute } from "./path";

describe("pathToRoute", () => {
  it("returns undefined if it can't parse the path", () => {
    expect(pathToRoute("/not/a/recognised/path")).toBeUndefined();
  });

  it("parses patch paths", () => {
    const route: Route = {
      type: "project",
      params: {
        urn: "rad:git:hnrk8ueib11sen1g9n1xbt71qdns9n4gipw1o",
        activeView: {
          type: "patch",
          peerId: "hyn5r6yejjco8r77yf7gu6gqsetgjsqt5oitpzu5eu791wej6p3xz6",
          id: "no-session-kv",
        },
      },
    };
    expect(pathToRoute(routeToPath(route))).toEqual(route);
  });
});
