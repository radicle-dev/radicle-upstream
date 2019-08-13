type page =
  | Root
  | Projects
  | NotFound(list(string));

let linkOfPage: page => string;
let nameOfPage: page => string;
let pageOfUrl: ReasonReactRouter.url => page;
