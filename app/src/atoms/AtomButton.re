module Styles = {
  open Css;
  open Particle;

  let primary =
    style([
      backgroundColor(Color.purple()),
      hover([backgroundColor(Color.purple(~alpha=0.85, ()))]),
      active([backgroundColor(Color.purple(~alpha=0.75, ()))]),
      borderRadius(px(2)),
      borderWidth(px(0)),
      outlineStyle(`none),
      padding4(~top=px(9), ~right=px(16), ~bottom=px(11), ~left=px(16)),
      color(Color.white()),
    ]);

  let secondary =
    merge([
      primary,
      style([
        backgroundColor(Color.blue()),
        hover([backgroundColor(Color.blue(~alpha=0.85, ()))]),
        active([backgroundColor(Color.blue(~alpha=0.75, ()))]),
      ]),
    ]);

  let disabled =
    merge([
      primary,
      style([
        backgroundColor(Color.grey()),
        hover([backgroundColor(Color.grey(~alpha=0.85, ()))]),
        active([backgroundColor(Color.grey(~alpha=0.75, ()))]),
      ]),
    ]);
};

[@react.component]
let make = (~children, ~style, ~disabled=false) =>
  <button className={disabled ? Styles.disabled : style} disabled>
    children
  </button>;
