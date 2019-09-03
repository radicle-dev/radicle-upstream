open Jest;
open Expect;
open ReasonReactRouter;
open Router;

describe("Router", () =>
  testAll(
    "pageOfUrl",
    [
      (Projects, {hash: "", path: [], search: ""}),
      (Projects, {hash: "", path: ["projects"], search: ""}),
      (
        RegisterProject,
        {hash: "", path: ["projects", "register"], search: ""},
      ),
      (
        Project("monokel"),
        {hash: "", path: ["projects", "monokel"], search: ""},
      ),
      (
        NotFound(["not-found"]),
        {hash: "", path: ["not-found"], search: ""},
      ),
      (
        NotFound(["utter", "crap"]),
        {hash: "", path: ["utter", "crap"], search: ""},
      ),
    ],
    ((page, url)) =>
    expect(pageOfUrl(url)) |> toEqual(page)
  )
);
