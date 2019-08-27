open ReasonReactRouter;

type page =
  | Root
  | Projects
  | Project(string)
  | RegisterProject
  | ProjectCode(string)
  | ProjectFunds(string)
  | NotFound(list(string));

let navigateOfPage = (p: page) => {
  let join = (parts: list(string)): string =>
    List.fold_left((a, b) => a ++ "/" ++ b, "", parts);

  let link =
    switch (p) {
    | Root => "/"
    | Projects => join(["projects"])
    | RegisterProject => join(["projects", "register"])
    | Project(id) => join(["projects", id])
    | ProjectCode(id) => join(["projects", id, "code"])
    | ProjectFunds(id) => join(["projects", id, "funds"])
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
  | ProjectCode(id) => "Project " ++ id ++ "Code"
  | ProjectFunds(id) => "Project " ++ id ++ "Funds"
  | NotFound(_path) => "Not Found"
  };

let pageOfUrl = (u: url): page =>
  switch (u.path) {
  | [] => Root
  | ["projects"] => Projects
  | ["projects", "register"] => RegisterProject
  | ["projects", id] => Project(id)
  | ["projects", id, "code"] => ProjectCode(id)
  | ["projects", id, "funds"] => ProjectFunds(id)
  | ["not-found"] => NotFound(u.path)
  | _ => NotFound(u.path)
  };

let currentPage = (): page => {
  let url = ReasonReactRouter.useUrl();
  pageOfUrl(url);
};
