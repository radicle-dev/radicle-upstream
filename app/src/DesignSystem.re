open Css;

module Operators = {
  let (<<) = (a, b) => merge([a, b]);

  let (<<?) = (a, b) =>
    switch (b) {
    | Some(b) => a << b
    | None => a
    };
};

module Layout = {
  let margin = (top, right, bottom, left) =>
    style([
      marginTop(px(top)),
      marginRight(px(right)),
      marginBottom(px(bottom)),
      marginLeft(px(left)),
    ]);

  let grid =
    style([
      display(grid),
      gridTemplateColumns([`repeat((`num(8), `fr(1.0)))]),
      gridTemplateRows([px(174), auto, px(96)]),
      gridRowGap(px(0)),
    ]);

  module Container = {
    open Operators;
    module Styles = {
      let left = style([display(`flex), flex(`num(1.0))]);

      let right =
        style([display(`flex), flex(`num(1.0)), justifyContent(flexEnd)]);

      let twoColumns = style([display(`flex), flex(`num(1.0))]);
    };

    [@react.component]
    let make = (~children, ~style=?) =>
      <div className={Css.style([]) <<? style}> children </div>;

    module TwoColumns = {
      [@react.component]
      let make = (~children) =>
        <div className=Styles.twoColumns>
          <div className=Styles.left> {fst(children)} </div>
          <div className=Styles.right> {snd(children)} </div>
        </div>;
    };
  };
};
