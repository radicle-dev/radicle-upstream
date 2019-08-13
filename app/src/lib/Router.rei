/** Routable pages which relate to larger context of features. **/
type page =
  | Root
  | Projects
  | Project(string)
  | ProjectCode(string)
  | ProjectFunds(string)
  | NotFound(list(string));

/** Reads the current url and return a matching page, or NotFound. **/
let currentPage: unit => page;

/** Given a page returns a function which navigates to it by pushing a new url
 ** onto the pushState.
 **/
let navigateOfPage: (page, 'a) => unit;

/** Returns a human readable string for the given page, which can be used in
 ** navigations and other linking references.
 **/
let nameOfPage: page => string;

/** Given a ReasonReactRouter.url returns a matching page, or NotFound **/
let pageOfUrl: ReasonReactRouter.url => page;
