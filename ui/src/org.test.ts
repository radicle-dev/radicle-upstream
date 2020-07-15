import { getOrg } from "./org";
import { idValidationStore } from "./id";
import { ValidationStatus } from "./validation";
import { get } from "svelte/store";
import { orgMock } from "./__mocks__/api";

jest.mock("./api");

describe("fetching an org", () => {
  it("returns an org", async () => {
    const promise = await getOrg("radicle");
    expect(promise).toEqual(orgMock);
  });
});

describe("validation", () => {
  it("properly initializes a store", () => {
    const validation = idValidationStore();
    validation.subscribe(state =>
      expect(state).toEqual({ status: ValidationStatus.NotStarted })
    );
  });

  it("updates the store correctly", () => {
    const validation = idValidationStore();

    validation.validate("notradicle");

    expect(get(validation)).toEqual({
      status: ValidationStatus.Loading,
      input: "notradicle",
    });

    process.nextTick(() => {
      expect(get(validation)).toEqual({
        status: ValidationStatus.Error,
        message: "Sorry, this one is no longer available",
      });
    });
  });

  it("rejects ids of the wrong format", () => {
    const validation = idValidationStore();

    // no empty input
    validation.validate("");
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "This field is required",
    });

    // no spaces
    validation.validate("no spaces");
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "It should match ^[a-z0-9][a-z0-9]+$",
    });

    // no special characters
    validation.validate("^^^inVaLiD***");
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "It should match ^[a-z0-9][a-z0-9]+$",
    });

    // no starting with an underscore or dash
    validation.validate("_nVaLiD");
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "It should match ^[a-z0-9][a-z0-9]+$",
    });

    // must meet minimum length
    validation.validate("x");
    expect(get(validation)).toEqual({
      status: ValidationStatus.Error,
      message: "It should match ^[a-z0-9][a-z0-9]+$",
    });
  });

  it("doesn't allow you to register an existing org id", () => {
    const validation = idValidationStore();

    validation.validate("radicle");
    process.nextTick(() => {
      expect(get(validation)).toEqual({
        status: ValidationStatus.Error,
        message: "Sorry, this one is no longer available",
      });
    });
  });
});
