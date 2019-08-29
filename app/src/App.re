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

[@react.component]
let make = () => {
  open DesignSystem;
  open Page;
  open Router;

  let page =
    switch (currentPage()) {
    | Root => <Generic title="Home of Oscoin" />
    | Projects => <Projects />
    | RegisterProject => <RegisterProject />
    | Project(id) => <Project id subPage=Project.Overview />
    | ProjectCode(id) => <Project id subPage=Project.Code />
    | ProjectFunds(id) => <Project id subPage=Project.Funds />
    | NotFound(_path) => <Generic title="Not Found" />
    };

  <Store.Provider>
    <Container style=Layout.grid>
      <Container style={Positioning.wideWidthCentered << margin(32, 0, 0, 0)}>
        <Topbar />
      </Container>
      page
    </Container>
  </Store.Provider>;
};
