export type Rad = number;
export type MicroRad = number;
export type Usd = number;

export const microRadToRad = (microRad: MicroRad): Rad => {
  return microRad / 1_000_000;
};

export const radToUsd = (rad: Rad): Usd => {
  return rad * 1.0;
};
