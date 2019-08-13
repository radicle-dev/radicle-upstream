open Jest;
open Expect;
open ReasonReactRouter;
open Router;

describe("Router", () =>
  testAll(
    "pageOfUrl",
    [
      (Root, {hash: "", path: [], search: ""}),
      (Projects, {hash: "", path: ["projects"], search: ""}),
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
