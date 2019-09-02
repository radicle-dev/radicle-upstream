open DesignSystem;

module Styles = {
  open Css;
  open Particle;

  let text =
    style([color(Color.darkGray()), display(inline)])
    << style(Particle.Font.text);

  let smallText =
    style([color(Color.darkGray()), display(inline)])
    << style(Particle.Font.smallText);

  let caption =
    style([color(Color.darkGray()), display(inline)])
    << style(Particle.Font.caption);
};

[@react.component]
let make = (~children, ~style=?) =>
  <p className={Styles.text <<? style}> children </p>;

module Small = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p className={Styles.smallText <<? style}> children </p>;
};

module Caption = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <p className={Styles.caption <<? style}> children </p>;
};
