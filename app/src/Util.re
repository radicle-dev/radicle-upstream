module CssHelper = {
  open Css;
  let (<<) = (a, b) => merge([a, b]);

  let margin = (top, right, bottom, left) =>
    style([
      marginTop(px(top)),
      marginRight(px(right)),
      marginBottom(px(bottom)),
      marginLeft(px(left)),
    ]);
};
