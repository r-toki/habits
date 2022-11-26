export type User = {
  id: string;
  displayName: string;
};

export type Habit = {
  id: string;
  name: string;
  archived: boolean;
  createdAt: string;
};

export type DailyRecord = {
  comment: string;
  recordedOn: string;
  habitDailyRecords: HabitDailyRecord[];
};

export type HabitDailyRecord = {
  done: boolean;
  archived: boolean;
  habitId: string;
  habitName: string;
};

export type CreateUserInput = {
  displayName: string;
};

export type CreateHabitInput = {
  name: string;
};

export type UpdateDailyRecord = {
  recordedOn: string;
  comment: string;
  habitDailyRecords: { done: boolean; habitId: string }[];
};

export type CreateHabitSwapInput = {
  habitId1: string;
  habitId2: string;
};
