export const find = <T extends { id: string }>(ary: T[], id: string): T => {
  return ary.find((e) => e.id == id)!;
};
