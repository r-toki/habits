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

export type CreateUserInput = {
  displayName: string;
};

export type CreateHabitInput = {
  name: string;
};
