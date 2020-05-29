import { Org } from "../org"
import { User } from "../user"

type MockedResponse = Org | User | null

const radicleMock = {
  id: "radicle",
  shareableEntityIdentifier: "radicle@123abcd.git",
  avatarFallback: {
    background: {
      r: 255, g: 67, b: 34
    },
    emoji: "🔥"
  }
}

// just to give an idea of how we'd stub the api with other endpoints
const userMock = {
  handle: "rafalca"
}

export const get = async (endpoint: string): Promise<MockedResponse> => {
  const [prefix, param] = endpoint.split("/")

  let response: MockedResponse

  switch (prefix) {
    case "orgs":
      response = param === "radicle" ? radicleMock : null
      break;
    case "user":
      response = userMock
      break;
  }


  return new Promise((resolve) => resolve(response))
}
