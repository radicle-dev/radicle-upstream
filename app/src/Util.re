module CssHelper = {
  open Css;
  let (<<) = (a, b) => merge([a, b]);
};
