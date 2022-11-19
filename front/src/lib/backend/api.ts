import { axios } from './axios';
import { CreateHabitInput, CreateUserInput, Habit, User } from './type';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);

export const getUser = () => axios.get<User>('user').then(({ data }) => data);

export const createUser = (input: CreateUserInput) => axios.post('user', input);

export const getHabits = () => axios.get<Habit[]>('user/habits').then(({ data }) => data);

export const createHabit = (input: CreateHabitInput) => axios.post('user/habits', input);

export const deleteHabit = (id: string) => axios.delete(`user/habits/${id}`);

export const archiveHabit = (id: string) => axios.post(`user/habits/${id}/archive`);

export const unarchiveHabit = (id: string) => axios.delete(`user/habits/${id}/archive`);
