import * as sinon from "sinon";
import { handleCustomProtocolInvocation } from "./nativeCustomProtocolHandler";

jest.useFakeTimers("modern");

beforeEach(() => {
  jest.runAllTimers();
});

describe("handleCustomProtocolInvocation", () => {
  it("passes valid URLs", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation(
      `radicle://link/v1/rad:git:hnrkj7qjesxx4omprbj5c6apd97ebc9e5izoo`,
      callback
    );
    expect(callback.callCount).toEqual(1);
  });

  it("rejects empty strings", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation("", callback);
    expect(callback.callCount).toEqual(0);
  });

  it("passes URLs that are exactly 1024 bytes long", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation(`radicle://${"x".repeat(1014)}`, callback);
    expect(callback.callCount).toEqual(1);
  });

  it("rejects handling URLs longer than 1024 bytes", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation(`radicle://${"x".repeat(1015)}`, callback);
    expect(callback.callCount).toEqual(0);
  });

  it("rejects URLs that are not prefixed with radicle://", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation("upstream://", callback);
    expect(callback.callCount).toEqual(0);
  });

  it("throttles incomming requests", () => {
    const callback = sinon.stub();
    handleCustomProtocolInvocation("radicle://", callback);
    handleCustomProtocolInvocation("radicle://", callback);
    handleCustomProtocolInvocation("radicle://", callback);
    expect(callback.callCount).toEqual(1);
  });
});
