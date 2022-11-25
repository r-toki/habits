export type User = {
  id: string;
  displayName: string;
};

export type Habit = {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  archivedAt: string | null;
};

export type DailyRecord = {
  id: string;
  comment: string;
  recordedOn: string;
  habitDailyRecords: HabitDailyRecord[];
};

export type HabitDailyRecord = {
  done: boolean;
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
