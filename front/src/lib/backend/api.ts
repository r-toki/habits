import { axios } from './axios';
import { CreateHabitInput, CreateUserInput, Habit, User } from './type';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);

export const getUser = () => axios.get<User>('user').then(({ data }) => data);

export const createUser = (input: CreateUserInput) => axios.post('user', input);

export type HabitsQuery = {
  archived: 'true' | 'false' | 'null';
};
export const getHabits = (habitsQuery: HabitsQuery) =>
  axios
    .get<Habit[]>(
      `user/habits${Object.entries(habitsQuery)
        .filter((v) => v[1] != 'null')
        .reduce((acc, curr, idx) => acc + (idx == 0 ? '?' : '&') + `${curr[0]}=${curr[1]}`, '')}`,
    )
    .then(({ data }) => data);

export const createHabit = (input: CreateHabitInput) => axios.post('user/habits', input);

export const deleteHabit = (id: string) => axios.delete(`user/habits/${id}`);

export const archiveHabit = (id: string) => axios.post(`user/habits/${id}/archive`);

export const unarchiveHabit = (id: string) => axios.delete(`user/habits/${id}/archive`);
