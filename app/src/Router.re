open ReasonReactRouter;

type page =
  | Root
  | JoinNetwork
  | Projects
  | Project(string)
  | RegisterProject
  | Styleguide
  | NotFound(list(string));

type overlay = (option(page), option(page));

let navigateToPage = (p: page) => {
  let join = (parts: list(string)): string =>
    List.fold_left((a, b) => a ++ "/" ++ b, "", parts);

  let link =
    switch (p) {
    | Root => "/"
    | JoinNetwork => join(["join-network"])
    | Projects => join(["projects"])
    | RegisterProject => join(["projects", "register"])
    | Project(id) => join(["projects", id])
    | Styleguide => "/styleguide"
    | NotFound(_path) => "/not-found"
    };

  _ => ReasonReactRouter.push(link);
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

let valueOfSearchParam = (search, key) => {
  let rawPairs = Js.String.split("&", search);
  let safePairs =
    rawPairs
    |> Array.map(pair =>
         switch (Js.String.split("=", pair)) {
         | [|key, value|] => (Some(key), Some(value))
         | [|key|] => (Some(key), None)
         | _ => (None, None)
         }
       );

  let maybePair = Belt.Array.getBy(safePairs, ((k, _)) => k == Some(key));

  switch (maybePair) {
  | Some((_key, value)) => value
  | None => None
  };
};

let overlayOfSearch = search => {
  let ov =
    valueOfSearchParam(search, "overlay")
    ->Belt.Option.mapWithDefault(None, path => Some(pageOfPath([path])));

  let last =
    valueOfSearchParam(search, "last")
    ->Belt.Option.mapWithDefault(None, path => Some(pageOfPath([path])));

  (ov, last);
};

let currentOverlay = (): overlay => {
  let url = ReasonReactRouter.useUrl();
  overlayOfSearch(url.search);
};

let currentPage = (): page => {
  let url = ReasonReactRouter.useUrl();
  pageOfPath(url.path);
};
