// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as session from "./session";

describe("waitUnsealed", () => {
  it("resolves if session is already unsealed", async () => {
    session.__test__.sessionStore.success({
      status: session.Status.UnsealedSession,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      identity: undefined as any,
    });
    await session.waitUnsealed();
  });

  it("resolves when the session becomes unsealed", async () => {
    const unsealed = session.waitUnsealed();
    session.__test__.sessionStore.success({
      status: session.Status.UnsealedSession,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      identity: undefined as any,
    });
    await unsealed;
  });
});
