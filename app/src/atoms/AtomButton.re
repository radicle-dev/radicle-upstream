module Styles = {
  open Css;
  open Particle;

  let primary =
    merge([
      style([
        backgroundColor(Color.purple()),
        hover([backgroundColor(Color.purple(~alpha=0.85, ()))]),
        active([backgroundColor(Color.purple(~alpha=0.75, ()))]),
        borderRadius(px(4)),
        borderWidth(px(0)),
        outlineStyle(`none),
        height(px(48)),
        padding4(
          ~top=px(13),
          ~right=px(24),
          ~bottom=px(14),
          ~left=px(24),
        ),
        color(Color.white()),
      ]),
      style(Font.title),
    ]);

  let secondary =
    merge([
      primary,
      style([
        backgroundColor(Color.pink()),
        hover([backgroundColor(Color.pink(~alpha=0.85, ()))]),
        active([backgroundColor(Color.pink(~alpha=0.75, ()))]),
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
