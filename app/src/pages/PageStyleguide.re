open Atom;
open DesignSystem;
open Molecule.Modal;
open Particle.Color;

module Styles = {
  open Css;

  let wrapper =
    style([
      width(pct(100.0)),
      height(pct(100.0)),
      backgroundColor(Particle.Color.white()),
      paddingLeft(px(160)),
    ]);

  let content =
    Positioning.gridFullCentered << style([paddingTop(px(145))]);

  let section =
    style([
      marginBottom(px(180)),
      display(`grid),
      gridTemplateColumns([px(342)]),
    ]);

  let firstColumn = style([gridColumnStart(1), marginRight(px(90))]);

  let secondColumn = style([gridColumnStart(2)]);

  let row = style([display(`flex), marginBottom(px(34))]);
};

module ColorSwatchTemplate = {
  open Css;

  module Styles = {
    let container = style([marginRight(px(24))]);

    let color =
      style([width(px(120)), height(px(120)), marginBottom(px(16))]);
  };

  [@react.component]
  let make = (~color, ~containerStyle=?, ~colorStyle=?) =>
    <El style={Styles.container <<? containerStyle}>
      <div
        className={
          Styles.color
          << style([backgroundColor(rgbaOfColor(color, 1.0))])
          <<? colorStyle
        }
      />
      <Title> {React.string(nameOfColor(color))} </Title>
      <Text.Small style={style([textTransform(`uppercase)])}>
        {React.string(hexOfColor(color))}
      </Text.Small>
    </El>;
};

module ColorSwatch = {
  [@react.component]
  let make = (~color) => <ColorSwatchTemplate color />;

  module HalfSize = {
    open Css;
    [@react.component]
    let make = (~color) =>
      <ColorSwatchTemplate color colorStyle={style([height(px(60))])} />;
  };
};

[@react.component]
let make = () =>
  <Portal>
    <El style=Styles.wrapper>
      <El style=Styles.content>
        <El style=Styles.section>
          <Title.Huge> {React.string("Oscoin Styleguide")} </Title.Huge>
        </El>
        <El style=Styles.section>
          <El style=Styles.firstColumn>
            <Title.Big> {React.string("Colors")} </Title.Big>
            <Text style={Css.style([Css.color(Particle.Color.gray())])}>
              {React.string("Primary, secondary and grays")}
            </Text>
          </El>
          <El style=Styles.secondColumn>
            <El style=Styles.row>
              <ColorSwatch color=Purple />
              <ColorSwatch color=Blue />
              <ColorSwatch color=Green />
              <ColorSwatch color=Orange />
              <ColorSwatch color=Red />
              <ColorSwatch color=Bordeaux />
            </El>
            <El style=Styles.row>
              <ColorSwatch color=LightBlue />
              <ColorSwatch color=Teal />
              <ColorSwatch color=LightGreen />
              <ColorSwatch color=Yellow />
              <ColorSwatch color=Pink />
            </El>
            <El style=Styles.row>
              <ColorSwatch.HalfSize color=Black />
              <ColorSwatch.HalfSize color=DarkGray />
              <ColorSwatch.HalfSize color=Gray />
              <ColorSwatch.HalfSize color=LightGray />
              <ColorSwatch.HalfSize color=AlmostWhite />
              <ColorSwatch.HalfSize color=White />
            </El>
          </El>
        </El>
      </El>
    </El>
  </Portal>;
