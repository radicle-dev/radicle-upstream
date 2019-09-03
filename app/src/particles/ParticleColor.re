open Css;

type t =
  | Purple
  | Blue
  | Green
  | Orange
  | Red
  | Bordeaux
  | LightBlue
  | Teal
  | LightGreen
  | Yellow
  | Pink
  | Black
  | DarkGray
  | Gray
  | LightGray
  | AlmostWhite
  | White;

let rgbaOfColor = (color, alpha) =>
  switch (color) {
  | Purple => rgba(110, 72, 221, alpha)
  | Blue => rgba(41, 146, 202, alpha)
  | Green => rgba(48, 210, 124, alpha)
  | Orange => rgba(244, 190, 69, alpha)
  | Red => rgba(229, 81, 84, alpha)
  | Bordeaux => rgba(189, 71, 108, alpha)
  | LightBlue => rgba(154, 161, 237, alpha)
  | Teal => rgba(89, 226, 243, alpha)
  | LightGreen => rgba(55, 241, 143, alpha)
  | Yellow => rgba(227, 235, 55, alpha)
  | Pink => rgba(222, 119, 201, alpha)
  | Black => rgba(40, 51, 61, alpha)
  | DarkGray => rgba(85, 100, 115, alpha)
  | Gray => rgba(145, 160, 174, alpha)
  | LightGray => rgba(206, 216, 225, alpha)
  | AlmostWhite => rgba(248, 248, 248, alpha)
  | White => rgba(255, 255, 255, alpha)
  };

let nameOfColor = color =>
  switch (color) {
  | Purple => "purple"
  | Blue => "blue"
  | Green => "green"
  | Orange => "orange"
  | Red => "red"
  | Bordeaux => "bordeaux"
  | LightBlue => "lightBlue"
  | Teal => "teal"
  | LightGreen => "lightGreen"
  | Yellow => "yellow"
  | Pink => "pink"
  | Black => "black"
  | DarkGray => "darkGray"
  | Gray => "gray"
  | LightGray => "lightGray"
  | AlmostWhite => "almostWhite"
  | White => "white"
  };

let hexOfColor = color =>
  switch (color) {
  | Purple => "#6e41e0"
  | Blue => "#1f91cc"
  | Green => "#20d479"
  | Orange => "#f5bf35"
  | Red => "#e75050"
  | Bordeaux => "#bf456b"
  | LightBlue => "#999ff0"
  | Teal => "#51e2f5"
  | LightGreen => "#24f38b"
  | Yellow => "#e3ed11"
  | Pink => "#e074cb"
  | Black => "#28333d"
  | DarkGray => "#546474"
  | Gray => "#90a0af"
  | LightGray => "#ced8e1"
  | AlmostWhite => "#f8f8f8"
  | White => "#ffffff"
  };

/** Primaries **/
let purple = (~alpha=1.0, ()) => rgbaOfColor(Purple, alpha);
let blue = (~alpha=1.0, ()) => rgbaOfColor(Blue, alpha);
let green = (~alpha=1.0, ()) => rgbaOfColor(Green, alpha);
let orange = (~alpha=1.0, ()) => rgbaOfColor(Orange, alpha);
let red = (~alpha=1.0, ()) => rgbaOfColor(Red, alpha);
let bordeaux = (~alpha=1.0, ()) => rgbaOfColor(Bordeaux, alpha);

/** Secondaries **/
let lightBlue = (~alpha=1.0, ()) => rgbaOfColor(LightBlue, alpha);
let teal = (~alpha=1.0, ()) => rgbaOfColor(Teal, alpha);
let lightGreen = (~alpha=1.0, ()) => rgbaOfColor(LightGreen, alpha);
let yellow = (~alpha=1.0, ()) => rgbaOfColor(Yellow, alpha);
let pink = (~alpha=1.0, ()) => rgbaOfColor(Pink, alpha);

/** Grays **/
let black = (~alpha=1.0, ()) => rgbaOfColor(Black, alpha);
let darkGray = (~alpha=1.0, ()) => rgbaOfColor(DarkGray, alpha);
let gray = (~alpha=1.0, ()) => rgbaOfColor(Gray, alpha);
let lightGray = (~alpha=1.0, ()) => rgbaOfColor(LightGray, alpha);
let almostWhite = (~alpha=1.0, ()) => rgbaOfColor(AlmostWhite, alpha);
let white = (~alpha=1.0, ()) => rgbaOfColor(White, alpha);
