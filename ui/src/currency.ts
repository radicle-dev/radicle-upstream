export type Rad = number;
export type MicroRad = number;
export type Usd = number;

export const microRadToRad = (microRad: MicroRad): Rad => {
  return microRad / 1_000_000;
};

export const radToUsd = (rad: Rad): Usd => {
  return rad * 1.0;
};

export const microRadToUsd = (microRad: MicroRad): Usd => {
  return microRadToRad(microRad) * 1;
};

export const radToMicroRad = (rad: Rad): MicroRad => {
  return rad * 1_000_000;
};
