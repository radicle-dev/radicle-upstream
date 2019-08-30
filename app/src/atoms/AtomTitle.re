open Particle.Font;
open DesignSystem;

module Huge = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <h1 className={Css.style(hugeTitle) <<? style}> children </h1>;
};

module Big = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <h2 className={Css.style(bigTitle) <<? style}> children </h2>;
};

[@react.component]
let make = (~children, ~style=?) =>
  <h3 className={Css.style(title) <<? style}> children </h3>;
