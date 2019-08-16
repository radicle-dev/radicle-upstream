module Styles = {
  open Css;

  global("body", [color(Particle.Color.black()), ...Particle.Font.text]);

  global(
    "a",
    [
      color(Particle.Color.black()),
      textDecoration(none),
      cursor(`pointer),
    ],
  );
};

[@react.component]
let make = () => {
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

  <div> <Topbar /> page <Footer /> </div>;
};
