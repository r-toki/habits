import { axios } from './axios';
import { CreateUserInput, User } from './type';

export const getIndex = () => axios.get<string>('').then(({ data }) => data);

export const getUser = () => axios.get<User>('user').then(({ data }) => data);

export const createUser = ({ displayName }: CreateUserInput) => axios.post('user', { displayName });
