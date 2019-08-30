open ReasonReactRouter;

type page =
  | Root
  | Projects
  | Project(string)
  | RegisterProject
  | NotFound(list(string));

let navigateToPage = (p: page) => {
  let join = (parts: list(string)): string =>
    List.fold_left((a, b) => a ++ "/" ++ b, "", parts);

  let link =
    switch (p) {
    | Root => "/"
    | Projects => join(["projects"])
    | RegisterProject => join(["projects", "register"])
    | Project(id) => join(["projects", id])
    | NotFound(_path) => "/not-found"
    };

  _ => ReasonReactRouter.push(link);
};

let nameOfPage = (p: page): string =>
  switch (p) {
  | Root => "Root"
  | Projects => "Explore"
  | RegisterProject => "Register Project"
  | Project(id) => "Project " ++ id
  | NotFound(_path) => "Not Found"
  };

let pageOfUrl = (u: url): page =>
  switch (u.path) {
  | [] => Projects
  | ["projects"] => Projects
  | ["projects", "register"] => RegisterProject
  | ["projects", id] => Project(id)
  | ["not-found"] => NotFound(u.path)
  | _ => NotFound(u.path)
  };

let currentPage = (): page => {
  let url = ReasonReactRouter.useUrl();
  pageOfUrl(url);
};
