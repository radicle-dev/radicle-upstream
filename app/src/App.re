module Styles = {
  open Css;

  global(
    "body",
    [
      unsafe("-webkit-font-smoothing", "antialiased"),
      unsafe("-moz-osx-font-smoothing", "grayscale"),
      color(Particle.Color.black()),
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

  let container =
    style([
      display(grid),
      unsafe("gridTemplateColumns", "repeat(8, 1fr)"),
      unsafe("grid-template-rows", "repeat(8, 1fr)"),
      gridGap(px(24)),
      height(vh(100.0)),
    ]);

  let content =
    style([
      marginTop(px(50)),
      gridColumnStart(3),
      gridColumnEnd(7),
      height(pct(100.0)),
    ]);
};

[@react.component]
let make = () => {
  open Page;
  open Router;

  let page =
    switch (currentPage()) {
    | Root => <Generic title="Home of Oscoin" />
    | Projects => <Projects />
    | Project(id) => <Project id subPage=Project.Overview />
    | ProjectCode(id) => <Project id subPage=Project.Code />
    | ProjectFunds(id) => <Project id subPage=Project.Funds />
    | NotFound(_path) => <Generic title="Not Found" />
    };

  <div className=Styles.container>
    <Topbar />
    <div className=Styles.content> page </div>
    <Footer />
  </div>;
};
