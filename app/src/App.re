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
};

let matchPage = page: React.element =>
  switch (page) {
  | Root => <Generic title="Home of Oscoin" />
  | JoinNetwork => <JoinNetwork />
  | Projects => <Projects />
  | RegisterProject => <RegisterProject />
  | Project(address) => <Project address />
  | Styleguide => <Styleguide />
  | NotFound(_path) => <Generic title="Not Found" />
  };

module Overlay = {
  open Molecule;

  [@react.component]
  let make = (~overlay, ~page) =>
    switch (overlay) {
    | (Some(overlayPage), _) =>
      let el = matchPage(overlayPage);

      <Modal onClose={navigateToPage(page)}> el </Modal>;
    | _ => React.null
    };
};

module SessionGuard = {
  open AppStore;
  open StoreSession;

  [@react.component]
  let make = (~children, ~overlay, ~page) => {
    let state = Store.useSelector(state => state.session);
    let (ov, _) = overlay;

    let hasOverlay =
      switch (ov) {
      | Some(_overlayPage) => true
      | None => false
      };

    if (state == Empty && page == RegisterProject && !hasOverlay) {
      navigateToOverlay(
        Projects,
        (Some(JoinNetwork), Some(RegisterProject)),
        (),
      );
    };

    children;
  };
};

[@react.component]
let make = () => {
  open DesignSystem;

  let page = matchPage(currentPage());

  currentPage() == Router.Styleguide ?
    page :
    <Store.Provider>
      <El style=Layout.grid>
        <El style={Positioning.gridWideCentered << margin(32, 0, 0, 0)}>
          <Topbar />
        </El>
        <SessionGuard overlay={currentOverlay()} page={currentPage()}>
          page
        </SessionGuard>
      </El>
      <Overlay overlay={currentOverlay()} page={currentPage()} />
    </Store.Provider>;
};
