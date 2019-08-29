open Css;

module Operators = {
  let (<<) = (a, b) => merge([a, b]);

  let (<<?) = (a, b) =>
    switch (b) {
    | Some(b) => a << b
    | None => a
    };
};

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
