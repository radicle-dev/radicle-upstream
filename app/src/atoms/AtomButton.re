module Styles = {
  open Css;
  open Particle;

  let button =
    merge([
      style([
        backgroundColor(Color.white()),
        borderRadius(px(4)),
        borderStyle(solid),
        borderWidth(px(1)),
        borderColor(Color.lightGray()),
        color(Color.darkGray()),
        cursor(`pointer),
        height(px(48)),
        outlineStyle(`none),
        padding4(
          ~top=px(13),
          ~right=px(24),
          ~bottom=px(14),
          ~left=px(24),
        ),
        hover([backgroundColor(Color.almostWhite(~alpha=0.85, ()))]),
        active([backgroundColor(Color.almostWhite(~alpha=0.2, ()))]),
      ]),
      style(Font.title),
    ]);

  let primaryButton =
    merge([
      button,
      style([
        borderStyle(none),
        color(Color.white()),
        backgroundColor(Color.purple()),
        hover([backgroundColor(Color.purple(~alpha=0.85, ()))]),
        active([backgroundColor(Color.purple(~alpha=0.75, ()))]),
      ]),
    ]);

  let secondaryButton =
    merge([
      button,
      style([
        borderStyle(none),
        color(Color.white()),
        backgroundColor(Color.pink()),
        hover([backgroundColor(Color.pink(~alpha=0.85, ()))]),
        active([backgroundColor(Color.pink(~alpha=0.75, ()))]),
      ]),
    ]);

  let disabled =
    merge([
      button,
      style([
        backgroundColor(Color.gray()),
        borderStyle(none),
        hover([backgroundColor(Color.gray())]),
        active([backgroundColor(Color.gray())]),
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
