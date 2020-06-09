export type Rad = number;
export type MicroRad = number;
export type Usd = number;

export const microRadToRad = (microRad: MicroRad): Rad => {
  return 1_000_000 / microRad;
};

export const RadToUsd = (rad: Rad): Usd => {
  return rad * 1.0;
}
