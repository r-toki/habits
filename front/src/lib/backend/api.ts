import { axios } from './axios';
import {
  CreateHabit,
  CreateHabitSwap,
  CreateMe,
  DailyRecord,
  Habit,
  Me,
  UpdateDailyRecord,
  UpdateHabit,
} from './type';

export const getMe = () => axios.get<Me>('me').then(({ data }) => data);

export const createMe = (input: CreateMe) => axios.post('me', input);

export type HabitsQuery = {
  archived?: boolean;
};
export const getHabits = (habitsQuery: HabitsQuery) =>
  axios.get<Habit[]>('habits' + toQueryString(habitsQuery)).then(({ data }) => data);

export const createHabit = (input: CreateHabit) => axios.post('habits', input);

export const updateHabit = ({ habitId, ...input }: UpdateHabit) =>
  axios.patch(`habits/${habitId}`, input);

export const deleteHabit = (id: string) => axios.delete(`habits/${id}`);

export const archiveHabit = (id: string) => axios.post(`habits/${id}/archive`);

export const swapHabits = (input: CreateHabitSwap) => axios.post('habits/swap', input);

export const getDailyRecord = (recordedOn: string) =>
  axios.get<DailyRecord | null>(`daily_records/${recordedOn}`).then(({ data }) => data);

export const updateDailyRecord = ({ recordedOn, ...input }: UpdateDailyRecord) =>
  axios.patch(`daily_records/${recordedOn}`, input);

const toQueryString = (query: Record<string, unknown>) =>
  Object.entries(query)
    .filter((v) => v[1] != undefined)
    .reduce((acc, curr, idx) => acc + (idx == 0 ? '?' : '&') + `${curr[0]}=${curr[1]}`, '');
