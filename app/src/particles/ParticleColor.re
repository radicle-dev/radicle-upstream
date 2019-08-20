open Css;

/** Primaries **/
let purple = (~alpha=1.0, ()) => rgba(110, 72, 221, alpha);
let blue = (~alpha=1.0, ()) => rgba(41, 146, 202, alpha);
let green = (~alpha=1.0, ()) => rgba(48, 210, 124, alpha);
let orange = (~alpha=1.0, ()) => rgba(244, 190, 69, alpha);
let red = (~alpha=1.0, ()) => rgba(229, 81, 84, alpha);
let bordeaux = (~alpha=1.0, ()) => rgba(189, 71, 108, alpha);

/** Secondaries **/
let lightBlue = (~alpha=1.0, ()) => rgba(154, 161, 237, alpha);
let teal = (~alpha=1.0, ()) => rgba(89, 226, 243, alpha);
let lightGreen = (~alpha=1.0, ()) => rgba(55, 241, 143, alpha);
let yellow = (~alpha=1.0, ()) => rgba(227, 235, 55, alpha);
let pink = (~alpha=1.0, ()) => rgba(222, 119, 201, alpha);

/** Grays **/
let black = (~alpha=1.0, ()) => rgba(40, 51, 61, alpha);
let darkGray = (~alpha=1.0, ()) => rgba(85, 100, 115, alpha);
let gray = (~alpha=1.0, ()) => rgba(145, 160, 174, alpha);
let lightGray = (~alpha=1.0, ()) => rgba(206, 216, 225, alpha);
let almostWhite = (~alpha=1.0, ()) => rgba(248, 248, 248, alpha);
let white = (~alpha=1.0, ()) => rgba(255, 255, 255, alpha);
