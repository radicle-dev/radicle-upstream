module Styles = {
  open Css;
  open Particle;

  let regular =
    merge([style([color(Color.darkGray())]), style(Particle.Font.text)]);
};

[@react.component]
let make = (~children) => <p className=Styles.regular> children </p>;
