open AppStore;
open DesignSystem;
open Molecule;
open Page;
open Router;

module Styles = {
  open Css;

  global(
    "body",
    [
      color(Particle.Color.black()),
      unsafe(" -webkit-font-smoothing", "antialiased"),
      unsafe(" -moz-osx-font-smoothing", "grayscale"),
      ...Particle.Font.text,
    ],
  );

  global(
    "a",
    [
      color(Particle.Color.black()),
      cursor(`pointer),
      textDecoration(none),
    ],
  );

  let topContainer =
    Positioning.gridWideCentered << style([marginTop(px(32)), zIndex(10)]);
};

let httpLink =
  ApolloLinks.createHttpLink(~uri="http://localhost:8080/graphql", ());
let client =
  ReasonApollo.createApolloClient(
    ~link=httpLink,
    ~cache=ApolloInMemoryCache.createInMemoryCache(),
    (),
  );

let elementOfPage = page: React.element =>
  switch (page) {
  | Root => <Generic title="Home of Oscoin" />
  | JoinNetwork => <JoinNetwork />
  | Projects => <Projects />
  | RegisterProject => <SessionGuard> <RegisterProject /> </SessionGuard>
  | Project(address) => <Project address />
  | Styleguide => <Styleguide />
  | NotFound(_path) => <Generic title="Not Found" />
  };

module Overlay = {
  [@react.component]
  let make = () => {
    let dispatch = Store.useDispatch();

    switch (Store.useSelector(state => state.overlay)) {
    | Some((overlay, last)) =>
      let el = elementOfPage(overlay);
      let onClose = _ev => {
        dispatch(OverlayAction(StoreOverlay.Hide));
        navigateToPage(last, ());
      };

      <Modal onClose> el </Modal>;
    | _ => React.null
    };
  };
};

[@react.component]
let make = () => {
  let page = elementOfPage(currentPage());

  currentPage() == Router.Styleguide ?
    page :
    <Store.Provider>
      <ReasonApolloHooks.ApolloProvider client>
        <El style=Layout.grid>
          <El style=Styles.topContainer> <Topbar /> <Alerts /> </El>
          page
        </El>
        <Overlay />
      </ReasonApolloHooks.ApolloProvider>
    </Store.Provider>;
};
