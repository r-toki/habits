import { axios } from './axios';
import {
  CreateHabitInput,
  CreateHabitSwapInput,
  CreateUserInput,
  DailyRecord,
  Habit,
  UpdateDailyRecord,
  UpdateHabitInput,
  User,
} from './type';

export const getUser = () => axios.get<User>('user').then(({ data }) => data);

export const createUser = (input: CreateUserInput) => axios.post('user', input);

export type HabitsQuery = {
  archived?: boolean;
};
export const getHabits = (habitsQuery: HabitsQuery) =>
  axios.get<Habit[]>('user/habits' + toQueryString(habitsQuery)).then(({ data }) => data);

export const createHabit = (input: CreateHabitInput) => axios.post('user/habits', input);

export const updateHabit = ({ habitId, ...input }: UpdateHabitInput) =>
  axios.patch(`user/habits/${habitId}`, input);

export const deleteHabit = (id: string) => axios.delete(`user/habits/${id}`);

export const archiveHabit = (id: string) => axios.post(`user/habits/${id}/archive`);

export const swapHabit = (input: CreateHabitSwapInput) => axios.post('user/habits/swap', input);

export const getDailyRecord = (recordedOn: string) =>
  axios.get<DailyRecord | null>(`user/daily_records/${recordedOn}`).then(({ data }) => data);

export const updateDailyRecord = ({ recordedOn, ...input }: UpdateDailyRecord) =>
  axios.patch(`user/daily_records/${recordedOn}`, input);

const toQueryString = (query: Record<string, unknown>) =>
  Object.entries(query)
    .filter((v) => v[1] != undefined)
    .reduce((acc, curr, idx) => acc + (idx == 0 ? '?' : '&') + `${curr[0]}=${curr[1]}`, '');
