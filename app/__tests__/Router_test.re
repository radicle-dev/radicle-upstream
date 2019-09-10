open Jest;
open Expect;
open Router;

describe("Router", () =>
  testAll(
    "pageOfUrl",
    [
      (Projects, []),
      (Projects, ["projects"]),
      (RegisterProject, ["projects", "register"]),
      (Project("monokel"), ["projects", "monokel"]),
      (Styleguide, ["styleguide"]),
      (NotFound(["not-found"]), ["not-found"]),
      (NotFound(["utter", "crap"]), ["utter", "crap"]),
    ],
    ((page, path)) =>
    expect(pageOfPath(path)) |> toEqual(page)
  )
);
