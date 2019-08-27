module Styles = {
  open Css;
  open Particle;
  open DesignSystem.Operators;

  let button =
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
      padding4(~top=px(13), ~right=px(24), ~bottom=px(14), ~left=px(24)),
      hover([backgroundColor(Color.almostWhite(~alpha=0.85, ()))]),
      active([backgroundColor(Color.almostWhite(~alpha=0.2, ()))]),
    ])
    << style(Font.title);

  let primaryButton =
    button
    << style([
         borderStyle(none),
         color(Color.white()),
         backgroundColor(Color.purple()),
         hover([backgroundColor(Color.purple(~alpha=0.85, ()))]),
         active([backgroundColor(Color.purple(~alpha=0.75, ()))]),
       ]);

  let secondaryButton =
    button
    << style([
         borderStyle(none),
         color(Color.white()),
         backgroundColor(Color.pink()),
         hover([backgroundColor(Color.pink(~alpha=0.85, ()))]),
         active([backgroundColor(Color.pink(~alpha=0.75, ()))]),
       ]);

  let cancelButton =
    button
    << style([
         borderStyle(none),
         color(Color.gray()),
         hover([
           backgroundColor(Color.white()),
           color(Color.gray(~alpha=0.75, ())),
         ]),
         active([
           backgroundColor(Color.white()),
           color(Color.gray(~alpha=0.55, ())),
         ]),
       ]);

  let disabled =
    button
    << style([
         backgroundColor(Color.gray()),
         borderStyle(none),
         hover([backgroundColor(Color.gray())]),
         active([backgroundColor(Color.gray())]),
       ]);
};

module Template = {
  [@react.component]
  let make = (~children, ~disabled, ~onClick, ~style) =>
    <button onClick className={disabled ? Styles.disabled : style} disabled>
      children
    </button>;
};

[@react.component]
let make = (~children, ~disabled=false, ~onClick=_ => ()) =>
  <Template disabled onClick style=Styles.button> children </Template>;

module Primary = {
  [@react.component]
  let make = (~children, ~disabled=false, ~onClick=_ => ()) =>
    <Template disabled onClick style=Styles.primaryButton>
      children
    </Template>;
};

module Secondary = {
  [@react.component]
  let make = (~children, ~disabled=false, ~onClick=_ => ()) =>
    <Template disabled onClick style=Styles.secondaryButton>
      children
    </Template>;
};

module Cancel = {
  [@react.component]
  let make = (~children, ~disabled=false, ~onClick=_ => ()) =>
    <Template disabled onClick style=Styles.cancelButton> children </Template>;
};
