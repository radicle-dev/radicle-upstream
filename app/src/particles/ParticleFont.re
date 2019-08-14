open Css;

let gtBold =
  fontFace(
    ~fontFamily="GT America",
    ~src=[url("/fonts/GTAmericaBold.otf")],
    ~fontStyle=normal,
    ~fontWeight=`normal,
    (),
  );

let gtMedium =
  fontFace(
    ~fontFamily="GT America",
    ~src=[url("/fonts/GTAmericaMedium.otf")],
    ~fontStyle=normal,
    ~fontWeight=`normal,
    (),
  );

let gtRegular =
  fontFace(
    ~fontFamily="GT America",
    ~src=[url("/fonts/GTAmericaRegular.otf")],
    ~fontStyle=normal,
    ~fontWeight=`normal,
    (),
  );

let gtMonoBold =
  fontFace(
    ~fontFamily="GT America Mono",
    ~src=[url("/fonts/GTAmericaMonoBold")],
    ~fontStyle=normal,
    ~fontWeight=`normal,
    (),
  );

let gtMonoRegular =
  fontFace(
    ~fontFamily="GT America Mono",
    ~src=[url("/fonts/GTAmericaMonoRegular")],
    ~fontStyle=normal,
    ~fontWeight=`normal,
    (),
  );

/** Titles **/

let hugeTitle = [
  fontFamily(gtRegular),
  fontSize(px(36)),
  lineHeight(px(43)),
];

let bigTitle = [
  fontFamily(gtRegular),
  fontSize(px(24)),
  lineHeight(px(29)),
];

let title = [
  fontFamily(gtRegular),
  fontSize(px(16)),
  lineHeight(`percent(130.0)),
];

/** Texts **/

let text = [fontFamily(gtRegular), fontSize(px(16)), lineHeight(px(24))];

let smallText = [
  fontFamily(gtRegular),
  fontSize(px(14)),
  lineHeight(px(20)),
];

/** Captions **/

let caption = [
  fontFamily(gtRegular),
  fontSize(px(13)),
  lineHeight(px(16)),
  letterSpacing(em(0.1)),
  textTransform(uppercase),
];

/** Numbers **/

let bigNumber = [
  fontFamily(gtMonoBold),
  fontSize(px(22)),
  lineHeight(px(27)),
];

let number = [
  fontFamily(gtMonoRegular),
  fontSize(px(16)),
  lineHeight(px(19)),
];

let smallNumber = [
  fontFamily(gtMonoRegular),
  fontSize(px(12)),
  lineHeight(px(14)),
];
