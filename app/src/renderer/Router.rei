/** Router implementation on top of ReasonReact Router.
 **
 ** All routing is done via changing the hash portion of the URL.
 ** This way our routes work when served by a webserver as well as in an
 ** Electron app when served from the file-system.
 **/

/** Routable pages which relate to larger context of features. **/
type page =
  | Root
  | JoinNetwork
  | Projects
  | Project(string)
  | RegisterProject
  | Styleguide
  | NotFound;

/** Reads the current url and returns a matching page.
 **
 ** Given: http://localhost:8000/#projects/monokel
 ** Returns: Project("monokel")
 **
 ** Given: file:///Users/rudolfs/work/mvp/app/build/index.html#projects/oscoin
 ** Returns: Project("oscoin")
 **
 ** Given: file:///Users/rudolfs/work/mvp/app/build/index.html#nonexistant/path
 ** Returns: NotFound
 **/
let currentPage: unit => page;

/** Navigates to a given page. This is done via pushState to update the
 ** browser's URL. Achtung, side-effects!
 **/
let navigateToPage: (page, 'a) => unit;

/** Returns a human readable string for the given page, which can be used in
 ** navigations and other linking references.
 **/
let nameOfPage: page => string;

/** Used only in tests. **/
let pageFromRoute: string => page;
