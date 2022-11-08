import { Tokens } from '../auth';
import { tokenStorage } from '../token-storage';
import { axios } from './axios';
import { CreateUserInput, User } from './type';

export const getIndex = async () => {
  await axios.get('');
};

export const getUser = async () => {
  const { data } = await axios.get<User>('user');
  return data;
};

export const createUser = async ({ name, password }: CreateUserInput) => {
  const { data } = await axios.post<Tokens>('user', { name, password });
  tokenStorage.set('access_token', data.accessToken);
  tokenStorage.set('refresh_token', data.refreshToken);
};
