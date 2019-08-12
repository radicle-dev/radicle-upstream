open Jest;

describe("Router", () =>
  Expect.(
    ReasonReactRouter.(
      Router.(
        test("pageOfUrl", () => {
          let url = {hash: "", path: ["projects"], search: ""};
          let page = pageOfUrl(url);

          expect(page) |> toBe(Projects);
        })
      )
    )
  )
);
