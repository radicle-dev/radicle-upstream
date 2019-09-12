open Atom;
open Particle;

type severity =
  | Info
  | Success
  | Error;

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

  let alert = severity =>
    switch (severity) {
    | Info => info
    | Success => success
    | Error => error
    };

  let close =
    style([
      cursor(`pointer),
      height(px(24)),
      marginLeft(auto),
      marginRight(px(8)),
    ]);

  let icon = style([height(px(24)), Css.margin(px(8))]);
};

module Template = {
  [@react.component]
  let make = (~children, ~severity, ~onClick=_ => ()) => {
    let icon =
      switch (severity) {
      | Info => <Icon.Info />
      | Success => <Icon.Important color=Color.Green />
      | Error => <Icon.Important color=Color.Bordeaux />
      };

    let closeIcon =
      switch (severity) {
      | Info => <Icon.CloseSmall />
      | Success => <Icon.CloseSmall color=Color.Green />
      | Error => <Icon.CloseSmall color=Color.Bordeaux />
      };

    <div className={Styles.alert(severity)}>
      <div className=Styles.icon> icon </div>
      <Title> children </Title>
      <div className=Styles.close onClick> closeIcon </div>
    </div>;
  };
};

[@react.component]
let make = (~children, ~severity=Info, ~onClick=_ => ()) =>
  <Template onClick severity> children </Template>;

module Success = {
  [@react.component]
  let make = (~children, ~onClick=_ => ()) =>
    <Template onClick severity=Success> children </Template>;
};

module Error = {
  [@react.component]
  let make = (~children, ~onClick=_ => ()) =>
    <Template onClick severity=Error> children </Template>;
};
