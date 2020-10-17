import { CSSPosition } from "./style";

export interface Position {
  x: number;
  y: number;
}

export enum Visibility {
  Hidden = "hidden",
  Visible = "visible",
}

export const calculatePosition = (
  position: CSSPosition,
  container: DOMRect,
  message: DOMRect
): Position => {
  const offsetY = container.height < 32 ? (32 - container.height) / 2 : 0;

  switch (position) {
    case CSSPosition.Top:
      return {
        x: container.left + container.width / 2,
        y: container.top - 40,
      };

    case CSSPosition.Right:
      return {
        x: container.right + 8,
        y: container.top - offsetY,
      };

    case CSSPosition.Bottom:
      return {
        x: container.left + container.width / 2,
        y: container.bottom + 8,
      };

    case CSSPosition.Left:
      return {
        x: container.left - message.width - 8,
        y: container.top + container.height / 2 - 16,
      };
  }
};
