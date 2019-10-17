open Css;

let (<<) = (a, b) => merge([a, b]);

let (<<?) = (a, b) =>
  switch (b) {
  | Some(b) => a << b
  | None => a
  };

let margin = (top, right, bottom, left) =>
  style([
    marginTop(px(top)),
    marginRight(px(right)),
    marginBottom(px(bottom)),
    marginLeft(px(left)),
  ]);

module Layout = {
  let grid =
    style([
      display(grid),
      gridTemplateColumns([`repeat((`num(8), `fr(1.0)))]),
      gridTemplateRows([px(174), auto, px(96)]),
      gridRowGap(px(0)),
    ]);

  let flex = style([display(`flex), flex(`num(1.0))]);
};

module Positioning = {
  let gridFullCentered = style([gridColumnStart(1), gridColumnEnd(9)]);
  let gridWideCentered = style([gridColumnStart(2), gridColumnEnd(8)]);
  let gridMediumCentered = style([gridColumnStart(3), gridColumnEnd(7)]);
  let gridNarrowCentered = style([gridColumnStart(4), gridColumnEnd(6)]);

  let flexLeft = style([display(`flex), flex(`num(1.0))]);
  let flexRight =
    style([display(`flex), flex(`num(1.0)), justifyContent(flexEnd)]);
  let flexCentered =
    style([display(`flex), flex(`num(1.0)), justifyContent(center)]);
};

module El = {
  [@react.component]
  let make = (~children, ~style=?) =>
    <div className={Css.style([]) <<? style}> children </div>;
};
