[@bs.val] [@bs.return nullable]
external _getElementById: string => option(Dom.element) =
  "document.getElementById";

module Styles = {
  open Css;
  open Particle;
  open DesignSystem;

  let modal =
    Layout.grid
    << style([
         position(fixed),
         width(pct(100.0)),
         height(pct(100.0)),
         backgroundColor(Color.white()),
         justifyContent(center),
       ]);

  let closeButton =
    style([
      cursor(`pointer),
      position(`absolute),
      top(px(0)),
      right(px(0)),
      padding(px(36)),
    ]);
};

module Portal = {
  [@react.component]
  let make = (~children) =>
    switch (_getElementById("portal")) {
    | None => raise(Not_found)
    | Some(modalElement) => ReactDOMRe.createPortal(children, modalElement)
    };
};

[@react.component]
let make = (~children, ~closeButtonCallback) =>
  <Portal>
    <div className=Styles.modal>
      <div className=Styles.closeButton onClick={_ => closeButtonCallback()}>
        <Atom.Icon.Close />
      </div>
      children
    </div>
  </Portal>;
