open ReasonReactRouter;

type page =
  | Root
  | JoinNetwork
  | Projects
  | Project(string)
  | RegisterProject
  | Styleguide
  | NotFound;

let nameOfPage = page =>
  switch (page) {
  | Root => "Root"
  | JoinNetwork => "Join Network"
  | Projects => "Explore"
  | RegisterProject => "Register Project"
  | Project(id) => "Project " ++ id
  | Styleguide => "Styleguide"
  | NotFound => "Not Found"
  };

let pageFromRoute = route => {
  let pathSegments = Js.String.split("/", route);

  switch (pathSegments) {
  | [|""|] => Projects
  | [|"join-network"|] => JoinNetwork
  | [|"projects"|] => Projects
  | [|"projects", "register"|] => RegisterProject
  | [|"projects", id|] => Project(id)
  | [|"styleguide"|] => Styleguide
  | [|"not-found"|] => NotFound
  | _ => NotFound
  };
};

let routeFromPage = page =>
  switch (page) {
  | Root => ""
  | JoinNetwork => "#join-network"
  | Projects => "#projects"
  | RegisterProject => "#projects/register"
  | Project(id) => "#projects/" ++ id
  | Styleguide => "#styleguide"
  | NotFound => "#not-found"
  };

let navigateToPage = (page, _) =>
  ReasonReactRouter.push(routeFromPage(page));

let currentPage = (): page => {
  let hashPartOfUrl = ReasonReactRouter.useUrl().hash;

  pageFromRoute(hashPartOfUrl);
};
