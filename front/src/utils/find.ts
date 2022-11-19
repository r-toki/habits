import { assertDefined } from './assert-defined';

export const find = <T extends { id: string }>(ary: T[], id: string) => {
  const res = ary.find((e) => e.id == id);
  assertDefined(res);
  return res;
};
