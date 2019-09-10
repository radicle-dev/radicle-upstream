open ReasonReactRouter;

type page =
  | Root
  | JoinNetwork
  | Projects
  | Project(string)
  | RegisterProject
  | Styleguide
  | NotFound(list(string));

let join = (parts: list(string)): string =>
  List.fold_left((a, b) => a ++ "/" ++ b, "", parts);

let linkOfUrl = url => {
  let path = join(url.path);
  let search = url.search != "" ? "?" ++ url.search : "";
  let hash = url.hash != "" ? "#" ++ url.hash : "";

  path ++ search ++ hash;
};

let nameOfPage = (p: page): string =>
  switch (p) {
  | Root => "Root"
  | JoinNetwork => "Join Network"
  | Projects => "Explore"
  | RegisterProject => "Register Project"
  | Project(id) => "Project " ++ id
  | Styleguide => "Styleguide"
  | NotFound(_path) => "Not Found"
  };

let pageOfPath = path: page =>
  switch (path) {
  | [] => Projects
  | ["join-network"] => JoinNetwork
  | ["projects"] => Projects
  | ["projects", "register"] => RegisterProject
  | ["projects", id] => Project(id)
  | ["styleguide"] => Styleguide
  | ["not-found"] => NotFound(path)
  | _ => NotFound(path)
  };

let pathOfPage = p =>
  switch (p) {
  | Root => ["/"]
  | JoinNetwork => ["join-network"]
  | Projects => ["projects"]
  | RegisterProject => ["projects", "register"]
  | Project(id) => ["projects", id]
  | Styleguide => ["styleguide"]
  | NotFound(_path) => ["not-found"]
  };

let navigateToPage = (p, _) =>
  ReasonReactRouter.push(
    linkOfUrl({path: pathOfPage(p), hash: "", search: ""}),
  );

let currentPage = (): page => {
  let url = ReasonReactRouter.useUrl();
  pageOfPath(url.path);
};
