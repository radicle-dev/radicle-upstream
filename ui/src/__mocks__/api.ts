import { Org } from "../org"

type MockedResponse = Org | boolean

export const get = async (endpoint: string): Promise<MockedResponse> => {
  const prefix = endpoint.split("/")[0]
  let response: MockedResponse

  switch (prefix) {
    case "orgs":
      response = {
        id: "radicle", shareableEntityIdentifier: "radicle@123abcd.git", avatarFallback: {
          background: {
            r: 22,
            g: 22,
            b: 22
          },
          emoji: "🎉"
        }
      }
  }

  return new Promise((resolve) => {
    process.nextTick(() => resolve(response))
  })
}