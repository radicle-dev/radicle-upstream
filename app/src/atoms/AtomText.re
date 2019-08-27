module Styles = {
  open Css;
  open Particle;
  open Util.CssHelper;

  let regular =
    style([color(Color.darkGray())]) << style(Particle.Font.text);
};

[@react.component]
let make = (~children) => <p className=Styles.regular> children </p>;
