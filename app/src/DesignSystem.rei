/** Shorthand for Css.merge, to be used for easy inline style overrides:
 **
 **   <El style={
 **     Styles.baseStyle <<
 **     Css.style([color(red)]) <<
 **     margin(0, 10, 0, 0)}>
 **   </El>
 **/
let (<<): (string, string) => string;

/** Optional Css.merge, to avoid boiler-plate in component declarations:
 **
 **   [@react.component]
 **   let make = (~children, ~style=?) =>
 **     <p className={Styles.smallText <<? style}> children </p>
 **/
let (<<?): (string, option(string)) => string;

/** Shorthand for setting margins, for ease of use:
 **
 **   <El style=margin(0, 10, 0, 0)> </El>
 **/
let margin: (int, int, int, int) => string;

module Layout: {
  /** CSS grid style, to be used for container elements in combination with
   ** the respective grid* Positioning styles for child elements:
   **
   **   <El style=Layout.grid>
   **     <El style={Positioning.gridWideCentered << margin(32, 0, 0, 0)}>
   **     </El>
   **   </El>
   **/
  let grid: string;

  /** Flexbox style, to be used for container elements in combination with
   ** the respective flex* Positioning styles for child elements:
   **
   **   <El style=Layout.flex>
   **     <El style=Positioning.flexLeft>
   **       {React.string("I'm on the left side")}
   **     </El>
   **     <El style=Positioning.flexRight>
   **       {React.string("I'm on the right side")}
   **     </El>
   **   </El>
   **/
  let flex: string;
};

/** Styles for containers that take different widths of the screen. **/
module Positioning: {
  /** Centered, taking full 8 columns. **/
  let gridFullCentered: string;

  /** Centered, 6 columns wide. **/
  let gridWideCentered: string;

  /** Centered, 4 columns wide. **/
  let gridMediumCentered: string;

  /** Centered, 2 columns wide. **/
  let gridNarrowCentered: string;

  /** To be used for child containers of a flex container. **/
  let flexLeft: string;
  let flexRight: string;
};

/** Basic container element for layouts, should be used instead of <div>,
 ** styles can be optionally passed down via the ~style prop.
 **/
module El: {
  [@bs.obj]
  external makeProps:
    (~children: 'children, ~style: 'style=?, ~key: string=?, unit) =>
    {
      .
      "children": 'children,
      "style": option('style),
    } =
    "";

  let make:
    {
      .
      "children": ReasonReact.reactElement,
      "style": option(string),
    } =>
    ReasonReact.reactElement;
};
