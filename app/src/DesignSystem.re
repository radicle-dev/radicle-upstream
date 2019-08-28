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
