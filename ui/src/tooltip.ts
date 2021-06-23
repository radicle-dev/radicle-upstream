// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

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
  switch (position) {
    case CSSPosition.Top:
      return {
        x: container.left + container.width / 2,
        y: container.top - 40,
      };

    case CSSPosition.Right:
      return {
        x: container.right + 8,
        y: container.top + container.height / 2 - 16,
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
