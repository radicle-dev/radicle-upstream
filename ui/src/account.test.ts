import { exists, balance } from "./account";
jest.mock("./api");

describe("account exists", () => {
  it("returns true when account exists", async () => {
    const aliceAaccountId = "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu";
    const promise = await exists(aliceAaccountId);
    expect(promise).toEqual(true);
  });
});

describe("account balance", () => {
  it("returns expected balance for existing account", async () => {
    const aliceAaccountId = "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu";
    const promise = await balance(aliceAaccountId);
    expect(promise).toEqual(1152921504606846976);
  });
});
