module Styles = {
  open Css;
  open Particle;

  let button =
    merge([
      style([
        color(Color.black()),
        backgroundColor(Color.white()),
        hover([backgroundColor(Color.white(~alpha=0.85, ()))]),
        active([backgroundColor(Color.white(~alpha=0.75, ()))]),
        borderRadius(px(4)),
        borderWidth(px(1)),
        outlineStyle(`none),
        height(px(48)),
        padding4(
          ~top=px(13),
          ~right=px(24),
          ~bottom=px(14),
          ~left=px(24),
        ),
      ]),
      style(Font.title),
    ]);

  let primaryButton =
    merge([
      button,
      style([
        color(Color.white()),
        backgroundColor(Color.purple()),
        borderColor(Color.purple()),
        hover([backgroundColor(Color.purple(~alpha=0.85, ()))]),
        active([backgroundColor(Color.purple(~alpha=0.75, ()))]),
      ]),
    ]);

  let secondaryButton =
    merge([
      button,
      style([
        color(Color.black()),
        borderWidth(px(1)),
        backgroundColor(Color.pink()),
        hover([backgroundColor(Color.pink(~alpha=0.85, ()))]),
        active([backgroundColor(Color.pink(~alpha=0.75, ()))]),
      ]),
    ]);

  let disabled =
    merge([
      button,
      style([
        backgroundColor(Color.grey()),
        hover([backgroundColor(Color.grey(~alpha=0.85, ()))]),
        active([backgroundColor(Color.grey(~alpha=0.75, ()))]),
      ]),
    ]);
};

[@react.component]
let make = (~children, ~disabled=false) =>
  <button className={disabled ? Styles.disabled : Styles.button} disabled>
    children
  </button>;

module Primary = {
  [@react.component]
  let make = (~children, ~disabled=false) =>
    <button
      className={disabled ? Styles.disabled : Styles.primaryButton} disabled>
      children
    </button>;
};

module Secondary = {
  [@react.component]
  let make = (~children, ~disabled=false) =>
    <button
      className={disabled ? Styles.disabled : Styles.secondaryButton} disabled>
      children
    </button>;
};
