import { axios } from './axios';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);
