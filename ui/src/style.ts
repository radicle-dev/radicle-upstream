// Shared types and functionality related to styling

export type ButtonVariant =
  | "vanilla"
  | "primary"
  | "secondary"
  | "transparent"
  | "outline"
  | "destructive"
  | "embedded";

export enum CSSPosition {
  Top = "top",
  Right = "right",
  Bottom = "bottom",
  Left = "left",
}

export function ellipsed(x: string, length: number = 10): string {
  return `${x.slice(0, length + 2)}...${x.slice(-length)}`;
}
