module Styles = {
  open Css;
  open Particle.Font;

  let hugeTitle = style(hugeTitle);
  let bigTitle = style(bigTitle);
  let title = style(title);
};

module Huge = {
  [@react.component]
  let make = (~children) => <h1 className=Styles.hugeTitle> children </h1>;
};

module Big = {
  [@react.component]
  let make = (~children) => <h2 className=Styles.bigTitle> children </h2>;
};

[@react.component]
let make = (~children) => <h3 className=Styles.title> children </h3>;
