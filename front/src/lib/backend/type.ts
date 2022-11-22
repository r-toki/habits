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
  createdAt: string;
  updatedAt: string;
  habitDailyRecords: HabitDailyRecord[];
};

export type HabitDailyRecord = {
  id: string;
  done: boolean;
  recordedOn: string;
  createdAt: string;
  updatedAt: string;
  name: string;
};

export type CreateUserInput = {
  displayName: string;
};

export type CreateHabitInput = {
  name: string;
};
