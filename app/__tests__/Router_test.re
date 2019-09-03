open Jest;
open Expect;
open Router;

describe("Router", () => {
  testAll(
    "pageOfUrl",
    [
      (Projects, []),
      (Projects, ["projects"]),
      (RegisterProject, ["projects", "register"]),
      (Project("monokel"), ["projects", "monokel"]),
      (NotFound(["not-found"]), ["not-found"]),
      (NotFound(["utter", "crap"]), ["utter", "crap"]),
    ],
    ((page, path)) =>
    expect(pageOfPath(path)) |> toEqual(page)
  );

  testAll(
    "overlayOfSearch",
    [
      ((None, None), ""),
      ((Some(JoinNetwork), None), "overlay=join-network"),
      (
        (Some(JoinNetwork), Some(Projects)),
        "overlay=join-network&last=projects",
      ),
    ],
    ((overlay, search)) =>
    expect(overlayOfSearch(search)) |> toEqual(overlay)
  );
});
