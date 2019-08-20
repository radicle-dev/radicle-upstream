type tuple2Children = (ReasonReact.reactElement, ReasonReact.reactElement);

module Container = {
  module Styles = {
    open Css;

    let left = style([display(`flex), flex(`num(1.0))]);

    let right =
      style([display(`flex), flex(`num(1.0)), justifyContent(flexEnd)]);

    let twoColumns = style([display(`flex), flex(`num(1.0))]);
  };

  module TwoColumns = {
    [@react.component]
    let make = (~children: tuple2Children) =>
      <div className=Styles.twoColumns>
        <div className=Styles.left> {fst(children)} </div>
        <div className=Styles.right> {snd(children)} </div>
      </div>;
  };
};
