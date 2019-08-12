open Jest;
open JestDom;
open Expect;
open ReactTestingLibrary;

let () =
  test("App renders heading", () =>
    <App />
    |> render
    |> getByText(~matcher=`Str("Home of Oscoin"))
    |> expect
    |> toBeInTheDocument
  );
