/* ----------------------------------- Me ----------------------------------- */
export type Me = {
  id: string;
  displayName: string;
};
export type CreateMe = {
  displayName: string;
};

/* ---------------------------------- Habits --------------------------------- */
export type Habit = {
  id: string;
  name: string;
  archived: boolean;
  createdAt: string;
  recentDoneList: boolean[];
};
export type GetHabits = {
  archived?: boolean;
};
export type CreateHabit = {
  name: string;
};
export type UpdateHabit = {
  habitId: string;
  name: string;
};
export type CreateHabitSwap = {
  habitId1: string;
  habitId2: string;
};

/* ------------------------------ DailyRecords ------------------------------ */
export type DailyRecord = {
  comment: string;
  recordedOn: string;
  habitDailyRecords: HabitDailyRecord[];
};
export type GetDailyRecord = {
  recordedOn: string;
  tz: string;
};
export type HabitDailyRecord = {
  done: boolean;
  archived: boolean;
  habitId: string;
  habitName: string;
};
export type UpdateDailyRecord = {
  recordedOn: string;
  comment: string;
  habitDailyRecords: { done: boolean; habitId: string }[];
};
