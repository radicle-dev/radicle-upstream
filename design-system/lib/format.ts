// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

// dayjs(1632733193970).format(TRANSACTION_TIMESTAMP_FORMAT) ->
// 27 Sep 2021 at 10:59
export const TRANSACTION_TIMESTAMP_FORMAT = "D MMM YYYY [at] HH:mm";

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
export function shortProjectUrn(value: string): string {
  return shorten(value.replace("rad:git:", ""), 8);
}

// hyyo6u8rhnuswory4c6symx471yseke74oq1myfesoig7zggcixejy ->
// hyyo6u8r…ggcixejy
export function shortPeerId(value: string): string {
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
    return `${shortPeerId(match[1])}@${match[2]}`;
  }

  return value;
}

// 07e57974a3b0aa77a92c2a605c72523c6f996215 ->
// 07e5797
export function shortCommitHash(value: string): string {
  return value.slice(0, 7);
}
