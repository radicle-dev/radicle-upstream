open ReasonReactRouter;

type page =
  | Home
  | Projects
  | NotFound(list(string));

let linkOfPage = (p: page): string =>
  switch (p) {
  | Home => "/"
  | Projects => "/projects"
  | NotFound(_path) => "/not-found"
  };

let nameOfPage = (p: page): string =>
  switch (p) {
  | Home => "Home"
  | Projects => "Projects"
  | NotFound(_path) => "Not Found"
  };

let pageOfUrl = (u: url): page =>
  switch (u.path) {
  | [] => Home
  | ["projects"] => Projects
  | ["not-found"] => NotFound(u.path)
  | _ => NotFound(u.path)
  };
