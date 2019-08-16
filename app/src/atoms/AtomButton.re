module Styles = {
  open Css;
  open Particle;

  let button =
    merge([
      style([
        color(Color.black()),
        backgroundColor(Color.white()),
        hover([backgroundColor(Color.almostWhite(~alpha=0.85, ()))]),
        active([backgroundColor(Color.almostWhite(~alpha=0.2, ()))]),
        borderRadius(px(4)),
        borderStyle(solid),
        borderWidth(px(1)),
        borderColor(Color.black()),
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
        borderStyle(none),
        backgroundColor(Color.grey()),
        hover([backgroundColor(Color.grey())]),
        active([backgroundColor(Color.grey())]),
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
