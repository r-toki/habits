import { axios } from './axios';
import {
  CreateHabit,
  CreateHabitSwap,
  CreateMe,
  DailyRecord,
  GetDailyRecord,
  GetHabits,
  Habit,
  Me,
  UpdateDailyRecord,
  UpdateHabit,
} from './type';

/* ----------------------------------- Me ----------------------------------- */
export const getMe = () => axios.get<Me>('me').then(({ data }) => data);

export const createMe = (f: CreateMe) => axios.post('me', f);

/* --------------------------------- Habits --------------------------------- */
export const getHabits = (q: GetHabits) =>
  axios.get<Habit[]>('habits' + toQueryString(q)).then(({ data }) => data);

export const createHabit = (f: CreateHabit) => axios.post('habits', f);

export const updateHabit = ({ habitId, ...f }: UpdateHabit) => axios.patch(`habits/${habitId}`, f);

export const deleteHabit = (id: string) => axios.delete(`habits/${id}`);

export const archiveHabit = (id: string) => axios.post(`habits/${id}/archive`);

export const swapHabits = (f: CreateHabitSwap) => axios.post('habits/swap', f);

/* ------------------------------ DailyRecords ------------------------------ */
export const getDailyRecord = ({ recordedOn, ...q }: GetDailyRecord) =>
  axios
    .get<DailyRecord | null>(`daily_records/${recordedOn}` + toQueryString(q))
    .then(({ data }) => data);

export const updateDailyRecord = ({ recordedOn, ...f }: UpdateDailyRecord) =>
  axios.patch(`daily_records/${recordedOn}`, f);

/* ---------------------------------- Utils --------------------------------- */
const toQueryString = (query: Record<string, unknown>) =>
  Object.entries(query)
    .filter((v) => v[1] != undefined)
    .reduce((acc, curr, idx) => acc + (idx == 0 ? '?' : '&') + `${curr[0]}=${curr[1]}`, '');
