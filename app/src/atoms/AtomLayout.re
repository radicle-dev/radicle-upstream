module Styles = {
  open Css;

  let left = style([flex(`num(1.0)), display(`flex)]);

  let right =
    style([flex(`num(1.0)), display(`flex), justifyContent(flexEnd)]);

  let twoColumns = style([display(`flex), flex(`num(1.0))]);
};

module Container = {
  module TwoColumns = {
    [@react.component]
    let make = (~children) =>
      <div className=Styles.twoColumns>
        <div className=Styles.left> {fst(children)} </div>
        <div className=Styles.right> {snd(children)} </div>
      </div>;
  };
};
