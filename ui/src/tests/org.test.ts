import test from "ava"
import { Org, orgIdValidationStore } from "../org"
import { rewiremock } from '../rewiremock'
import { get } from 'svelte/store'
import { ValidationStatus } from '../validation'

// doesn't actually work this simply out of the box
// rewiremock("../api").with("../__mocks__/api.ts")

const radicle: Org = {
  id: "radicle",
  avatarFallback: {
    background: {
      r: 22,
      g: 22,
      b: 22
    },
    emoji: "9"
  },
  shareableEntityIdentifier: "fjdklasfjds"
}

const notradicle: Org = {
  id: "radcle",
  avatarFallback: {
    background: {
      r: 22,
      g: 23,
      b: 22
    },
    emoji: "9"
  },
  shareableEntityIdentifier: "fjdklasfjds"
}

test("org", t => {
  t.is(radicle, radicle)
  t.not(radicle, notradicle)

  const validation = orgIdValidationStore()
  validation.validate("anamethatshouldwork")
  t.deepEqual(get(validation), { status: ValidationStatus.Loading, input: "anamethatshouldwork" })
})
