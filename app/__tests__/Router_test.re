open Jest;
open Expect;
open Router;

describe("Router", () =>
  testAll(
    "pageFromRoute",
    [
      ("", Projects),
      ("join-network", JoinNetwork),
      ("projects", Projects),
      ("projects/register", RegisterProject),
      ("projects/monokel", Project("monokel")),
      ("styleguide", Styleguide),
      ("not-found", NotFound),
      ("this/page/does/not/exist", NotFound),
    ],
    ((route, page)) =>
    expect(pageFromRoute(route)) |> toEqual(page)
  )
);
