// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

export function shorten(
  value: string,
  beginningLength: number,
  endingLength?: number
): string {
  const beginning = value.slice(0, beginningLength);
  const ending = value.slice(-(endingLength ?? beginningLength));

  return `${beginning}…${ending}`;
}

// rad:git:hnrkjhtohoe3u9mmtqgc6apbzomwwpos9h7ky ->
// hnrkjhto…pos9h7ky
export function shortUrn(value: string): string {
  return shorten(value.replace("rad:git:", ""), 8);
}

// hyyo6u8rhnuswory4c6symx471yseke74oq1myfesoig7zggcixejy ->
// hyyo6u8r…ggcixejy
export function shortDeviceId(value: string): string {
  return shorten(value, 8);
}

// 0xA66A5686D5c3A42C0b6c76FEd05e58C6bc851E9f ->
// 0xa66a56…851e9f
export function shortEthAddress(value: string): string {
  return shorten(value.toLowerCase(), 8, 6);
}
export const shortEthTx = shortEthAddress;

// hynewpywqj6x4mxgj7sojhue3erucyexiyhobxx4du9w66hxhbfqbw@seedling.radicle.xyz:12345 ->
// hynewpyw…hxhbfqbw@seedling.radicle.xyz:12345
export function shortSeedAddress(value: string): string {
  const match = value.match(/^(.{54})@(.*)$/);

  if (match && match[1] && match[2]) {
    return `${shortDeviceId(match[1])}@${match[2]}`;
  }

  return value;
}
