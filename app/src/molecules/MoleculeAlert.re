open Atom;
open Particle;

module Styles = {
  open Css;
  open DesignSystem;

  let info =
    style([
      backgroundColor(Color.gray(~alpha=0.1, ())),
      borderWidth(px(1)),
      borderColor(Color.gray()),
      borderStyle(solid),
      borderRadius(px(2)),
      color(Color.gray()),
      height(px(40)),
      display(`flex),
      alignItems(center),
    ]);

  let success =
    info
    << style([
         backgroundColor(Color.green(~alpha=0.1, ())),
         borderColor(Color.green()),
         color(Color.green()),
       ]);

  let error =
    info
    << style([
         backgroundColor(Color.bordeaux(~alpha=0.1, ())),
         borderColor(Color.bordeaux()),
         color(Color.bordeaux()),
       ]);

  let close =
    style([
      cursor(`pointer),
      height(px(24)),
      marginLeft(auto),
      marginRight(px(8)),
    ]);

  let icon = style([height(px(24)), Css.margin(px(8))]);
};

[@react.component]
let make = (~children, ~onClick=_ => ()) =>
  <div className=Styles.info>
    <div className=Styles.icon> <Icon.Info /> </div>
    <Title> children </Title>
    <div className=Styles.close onClick> <Icon.CloseSmall /> </div>
  </div>;

module Success = {
  [@react.component]
  let make = (~children, ~onClick=_ => ()) =>
    <div className=Styles.success>
      <div className=Styles.icon> <Icon.Important color=Color.Green /> </div>
      <Title> children </Title>
      <div className=Styles.close onClick>
        <Icon.CloseSmall color=Color.Green />
      </div>
    </div>;
};

module Error = {
  [@react.component]
  let make = (~children, ~onClick=_ => ()) =>
    <div className=Styles.error>
      <div className=Styles.icon>
        <Icon.Important color=Color.Bordeaux />
      </div>
      <Title> children </Title>
      <div className=Styles.close onClick>
        <Icon.CloseSmall color=Color.Bordeaux />
      </div>
    </div>;
};
