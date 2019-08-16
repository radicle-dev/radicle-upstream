module Styles = {
  open Css;

  global(
    "body",
    [
      color(Particle.Color.black()),
      unsafe("-webkit-font-smoothing", "antialiased"),
      unsafe("-moz-osx-font-smoothing:", "grayscale"),
      ...Particle.Font.text,
    ],
  );

  global(
    "a",
    [
      color(Particle.Color.black()),
      textDecoration(none),
      cursor(`pointer),
    ],
  );

  let app = style([]);
};

[@react.component]
let make = () => {
  open Page;
  open Router;
  open Molecule;

  let page =
    switch (currentPage()) {
    | Root => <Generic title="Home of Oscoin" />
    | Projects => <Projects />
    | Project(id) => <Project id subPage=Project.Overview />
    | ProjectCode(id) => <Project id subPage=Project.Code />
    | ProjectFunds(id) => <Project id subPage=Project.Funds />
    | NotFound(_path) => <Generic title="Not Found" />
    };

  <div className=Styles.app> <Topbar /> page <Footer /> </div>;
};
