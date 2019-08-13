open ReasonReactRouter;

type page =
  | Root
  | Projects
  | NotFound(list(string));

let linkOfPage = (p: page): string =>
  switch (p) {
  | Root => "/"
  | Projects => "/projects"
  | NotFound(_path) => "/not-found"
  };

let nameOfPage = (p: page): string =>
  switch (p) {
  | Root => "Root"
  | Projects => "Projects"
  | NotFound(_path) => "Not Found"
  };

let pageOfUrl = (u: url): page =>
  switch (u.path) {
  | [] => Root
  | ["projects"] => Projects
  | ["not-found"] => NotFound(u.path)
  | _ => NotFound(u.path)
  };
