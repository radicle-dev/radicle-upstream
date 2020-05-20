import { getOrg } from "./org"

jest.mock("./api")

describe("fetching an org", () => {
  it("returns an org", async () => {

    const promise = await getOrg("radicle")
    expect(promise).toEqual({
      id: "radicle",
      shareableEntityIdentifier: "radicle@123abcd.git",
      avatarFallback: {
        background: {
          r: 22,
          g: 22,
          b: 22
        },
        emoji: "🎉"
      }
    })
  })
})
