import { getOrg, orgIdValidationStore } from "./org"
import { ValidationStatus } from './validation'
import { get } from 'svelte/store'

jest.mock("./api")

describe("fetching an org", () => {
  it("returns an org", async () => {

    const promise = await getOrg("radicle")
    expect(promise).toEqual({
      id: "radicle",
      shareableEntityIdentifier: "radicle@123abcd.git",
      avatarFallback: {
        background: {
          r: 255, g: 67, b: 34
        },
        emoji: "🔥"
      },
      members: [{handle: "rafalca"}]
    })
  })
})

describe("validation", () => {
  it("properly initializes a store", () => {
    const validation = orgIdValidationStore()
    validation.subscribe(state =>
      expect(state).toEqual({ status: ValidationStatus.NotStarted })
    )
  })

  it("updates the store correctly", () => {
    const validation = orgIdValidationStore()

    validation.validate("notradicle")

    expect(get(validation)).toEqual({ status: ValidationStatus.Loading, input: "notradicle" })

    process.nextTick(() => {
      expect(get(validation)).toEqual({ status: ValidationStatus.Success })
    })
  })

  it("rejects ids of the wrong format", () => {
    const validation = orgIdValidationStore()

    // no empty input
    validation.validate("")
    expect(get(validation)).toEqual({ status: ValidationStatus.Error, message: "Org id is required" })

    // no spaces
    validation.validate("no spaces")
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "Org id should match [a-z0-9][a-z0-9_-]+"
    })

    // no special characters
    validation.validate("^^^inVaLiD***")
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "Org id should match [a-z0-9][a-z0-9_-]+"
    })

    // no starting with an underscore or dash
    validation.validate("_nVaLiD")
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "Org id should match [a-z0-9][a-z0-9_-]+"
    })

    // must meet minimum length
    validation.validate("x")
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "Org id should match [a-z0-9][a-z0-9_-]+"
    })
  })

  it("doesn't allow you to register an existing org id", () => {
    const validation = orgIdValidationStore()

    validation.validate("radicle")
    process.nextTick(() => {
      expect(get(validation)).toEqual({
        status: ValidationStatus.Error,
        message: "Sorry, this id is already taken"
      })
    })
  })
})
