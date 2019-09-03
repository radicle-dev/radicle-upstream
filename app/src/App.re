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
  let make = () =>
    switch (currentOverlay()) {
    | (Some(overlayPage), None) =>
      let page = matchPage(overlayPage);

      <Modal onClose=(_ => Js.log("closed"))> page </Modal>;
    | _ => React.null
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
        page
      </El>
      <Overlay />
    </Store.Provider>;
};
